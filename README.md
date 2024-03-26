![Static Badge](https://img.shields.io/badge/Ubuntu-True-blue)
![Static Badge](https://img.shields.io/badge/Windows-False-red)
![Static Badge](https://img.shields.io/badge/Language-Rust-purple)
![Static Badge](https://img.shields.io/badge/For-Novice-brown)
---
#### 使用rust👾实现一个compiler💻 ;
---

#### 代码逻辑来自 [Crafting-Interpreters][1] ( 原代码为java实现 ) ;

#### 代码实现参考 Youtube上的 [rust 实现版本][2] ;
---
### Content:

1. #### Scanner

   String => Vec[Token]

   不断的扫描当前字符，或peek下一个字符，来判断当前的这个word是哪一个token ;
   
   ![scan_tokens](https://github.com/superbignut/ltl-compiler/blob/master/sources/scan_tokens.png)

2. #### Representing Code
   
   Expr => String

   使用枚举类型 Expr 来代表抽象语法树 AST 的节点，并实现可以递归将 AST 转换为 String 的函数 ;


   ![represent_code](https://github.com/superbignut/ltl-compiler/blob/master/sources/represent_code.png)


3. #### Parsing Expressions

   Vec[Token] => Expr

   使用递归下降法，逐步将一组 Token 匹配成一个语法树 Expr ; 具体匹配规则如下，越向下优先级越高：

         最顶层-表达式： expression -> equality

         等式表达式： equality -> comparision ( ("!=" | "==") comparision  ) * ;

         不等表达式： comparision -> term ( ( ">" | ">=" | "<" | "<=") ) * ;

         加法表达式： term -> factor ( ( "-" | "+" ) factor ) * ;

         乘法表达式： factor -> unary ( ( "/" | "*") unary ) * ;

         一元表达式： unary -> ( ( "!" | "-" ) unary ) | primary ;
         
         最底层-基础单元： primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
   
   并且，这里的规则实现，刻意的避免了前缀表达式的写法 ; 递归下降法的代码实现十分巧妙，部分如下：

         fn equality(&mut self) -> Result<Expr, String> {

            let mut expr = self.comparision()?;

            while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {

               let operator = self.previous();
               let right_expr = self.comparision()?;

               expr = Expr::Binary {
                     left: Box::new(expr),
                     operator,
                     right: Box::new(right_expr),
                  };
            }
            Ok(expr)
         }
   
   其中 equality、comparision、term 和 factor 函数的实现几乎一样，函数中的 while 对应着正则表达式中的 "*" ;


   ![represent_code](https://github.com/superbignut/ltl-compiler/blob/master/sources/parser1.png)


   unary 函数中没有循环，而是使用 if 来进行一次判断 ; 而 primary 函数则是对应着最基本的元素和 使用 "( )" 的情况，它们有着最高的优先级 ; 

   ![represent_code](https://github.com/superbignut/ltl-compiler/blob/master/sources/parser2.png)

   代码写到这里，可以发现 "=" 等于号暂时没有被考虑进去，但是如果将 "=" 写进去，比如：

         let sources = "1.0 * 3.0 * 2.0 + 2.0 * 4.1 = 14.0".to_string();
         
   仍然会被成功的解析，原因是由于没有任何一个函数会和 "=" 匹配到，"=" 和后面的 token 都会被省略掉，最后这个表达式只会返回前面部分的AST ; Eof 也是因为同样的道理被忽略 ;

4. #### Evaluating Expressions

   Expr => ExprLiteral

   对一个包含四则运算，比较，括号，取非的语法树 Expr 求值，返回得到的结果 ;

   得益于第三节已经构建好了语法树 AST，因此求值只需要不断匹配 AST 根节点的运算符，并递归当前节点的左右分支 ;

   ![interpreter](https://github.com/superbignut/ltl-compiler/blob/master/sources/interpreter.png)

   到现在为止，已经完成了一个类似于计算器的功能 ; 但还只支持一条语句 ;

5. #### Statements and State 
   
   Var 和 Print 语句 :

   增加 Var 定义式语句 、Print 输出语句，再结合最初的简单表达式语句 ，现在有三种基本的语句形式：

   ![interpreter](https://github.com/superbignut/ltl-compiler/blob/master/sources/statement.png)

   因此，parser 的结果不再是一个简单的 Expr 语法树，而应该上升到更高的语句层次 ; 并且，除了 Var 语句的变量定义， 为了让
   变量被定义后，也可以出现在表达式中并被正确 parse 进语法树，还需要在 Expr 中加入代表变量的一项，并对应修改 primary函数 ; 

   ![interpreter](https://github.com/superbignut/ltl-compiler/blob/master/sources/stmt.png)


   对三种 statement 求值时，print 语句需要打印表达式的值 ; 而 Var 语句则需要将变量和对应的初始值存储起来，进而可以在之后，解析到该变量的时候，将对应的值取出 ; 这个存储的数据结构选用的则是哈希表 ; 


   赋值语句 :
   
   赋值语句是优先级最低的表达式，并需要保证左侧是 l_value 的硬性要求，可有如下 parser 部分代码：

         fn assignment(&mut self) -> Result<Expr, String> {
            let expr = self.equality()?;
            if self.match_tokens(&[TokenType::Equal]) {     // 是否是 "=" ?

               let value = self.assignment()?;              // 允许 a = b = c，从右到左               

               if let Expr::Variable { name } = expr {      // 判断等号左侧是不是变量                     
                  return Ok(Expr::Assign {                  // Expr::Assign{name:Token, value:Box<Expr>}
                     name,                                  // 新的 Expr 类型
                     value: Box::new(value),
                  });
               } 
            }
            Ok(expr)
         }
   相应的是对赋值语句的解析部分： 当遇到赋值语句时，要对存储变量的 HashMap 的值进行修改：

         Expr::Assign { name, value } => {
            let new_value = self.evaluate(value)?;
            self.environment.assign(name, new_value.clone())?;
            Ok(new_value)
         }
   作用域 :

   parser 部分将每一个大括号的所有内容解析为一个语句的集合：Vec[Stmt] ; 
   
   解析部分则要对每一个大括号维护一个变量空间，对应之前提到的哈希表，用来存储局部变量的值，并需要实现变量遮蔽和向外查找的基本功能 ; 


   ![interpreter](https://github.com/superbignut/ltl-compiler/blob/master/sources/scope.png)


6. #### Control Flow

   if 和 while 的 parse 部分在 Stmt 中新建了 If_statement 和 While_statement 两种语句类型，并通过关键字和分隔符进行匹配 ; 解析部分值的注意的是 while 的判断部分需要每次更新 ; "or"  和 "and" 的优先级要比等式判断更低 : 

    
         assignment -> Identifier "=" assignment | logic_or

         logic_or -> logic_and ( "or" logic_and) *

         logic_and -> equality ( "and" equality) *

         equality -> comparision ( ("!=" | "==") comparision  ) *

   进而是 for 的实现， 不需要再次增加 Stmt 实现，而是作为 while 的语法糖进行转换 :


   ![interpreter](https://github.com/superbignut/ltl-compiler/blob/master/sources/sugar.png)


7. #### Functions

   函数的实现包括函数声明和函数调用两个部分，首先是 parser 部分，类似于变量声明，函数声明通过在语句层面增加 Stmt::Function 来实现; 而函数调用则通过在表达式层面增加 Expr::Call 来实现 ;


   ![interpreter](https://github.com/superbignut/ltl-compiler/blob/master/sources/fun_parser.png)


   进而是解析部分，解析函数声明语句时，需要在变量空间中增加函数名字到可调用函数对象的映射，也就是将形式参数，和使用形式参数的语句构成的 block 重新封装起来，在附加可调用的 Trait 后，像普通变量一样插入哈希表 ; 
   
   除了添加特征之外，这次对可调用对象的封装，与最初的由 parser 给出的封装的内容是一致的，一定程度上实现了前后端解耦合 ;


   ![interpreter](https://github.com/superbignut/ltl-compiler/blob/master/sources/decouple.png)


   在已经对函数声明解析完成的基础上，解析函数调用语句时只需要根据函数名字取出哈希表中的可调用对象，并作用在实参上即可 ;




   在rust的实现过程中，函数调用对象是通过 impl Callable 来进行实现，这个特征对象的关键需求就是实现了可被调用的 call 函数 :

      1. 创建函数私有的变量空间 ;
      2. 将函数声明时的形参和调用时的实参进行匹配，并插入到私有变量空间 ;
      3. 调用封装好的可调用对象，在私有空间中解析 "大括号" 中的语句 ;

   ![interpreter](https://github.com/superbignut/ltl-compiler/blob/master/sources/call.png)


   比较复杂的地方是返回值的添加，我们使用 Result 的第一个 Ok 进行类似于 Err 一样的向外传递。 需要注意的是，不仅是函数，包括控制流的 if，while，还有 block 的内部，即所有调用解析 Stmt 的函数 execute() 和 解析 Vec[ Stmt ]的函数 interpreter() 的地方都需要有是否 return 的判断，进而可以跳出解析过程。

8. #### Todo
   1. 关于全局变量、全局函数在函数内外的可见性问题
   2. Environment 中的 enclosing 的更新问题
   3. 函数声明和函数调用时，环境的更新、闭包的捕获问题
   4. 闭包调用时对捕获变量修改的更新问题

[1]:https://craftinginterpreters.com/
[2]:https://www.youtube.com/playlist?list=PLj_VrUwyDuXS4K3n7X4U4qmkjpuA8rJ76