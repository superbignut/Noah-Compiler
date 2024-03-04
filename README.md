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
         
   仍然会被成功的解析，原因是由于没有任何一个函数会和 "=" 匹配到，"=" 和后面的 token 都会被省略掉，最后这个表达式只会返回前面部分的AST ; Eof 也是因为同样的道理被忽略。

4. #### Evaluating Expressions

   Expr => ExprLiteral

   对一个包含四则运算，比较，括号，取非的语法树 Expr 求值，返回得到的结果。

   得益于第三节已经构建好了语法树 AST，因此求值只需要不断匹配 AST 根节点的运算符，并递归当前节点的左右分支。

   ![interpreter](https://github.com/superbignut/ltl-compiler/blob/master/sources/interpreter.png)

   到现在为止，已经完成了一个类似于计算器的功能 ; 但还只支持一条语句 ;

5. #### Statements and State (8.1 ~ 8.3)

   增加 Var 定义式语句 、Print 输出语句，再结合最初的简单表达式语句 ，现在有三种基本的语句形式：

   ![interpreter](https://github.com/superbignut/ltl-compiler/blob/master/sources/statement.png)

   因此，parser 的结果不再是一个简单的 Expr 语法树，而应该上升到更高的语句层次 ; 并且，除了 Var 语句的变量定义， 为了让
   变量被定义后，也可以出现在表达式中并被正确 parse 进语法树，还需要在 Expr 中加入代表变量的一项，并对应修改 primary函数 ; 

   ![interpreter](https://github.com/superbignut/ltl-compiler/blob/master/sources/stmt.png)


   对三种 statement 求值时，print 语句需要打印表达式的值 ; 而 Var 语句则需要将变量和对应的初始值存储起来，进而可以在之后，解析到该变量的时候，将对应的值取出 ; 这个存储的数据结构选用的则是哈希表 ; 


   进而添加赋值语句，赋值语句是优先级最低的表达式，并需要保证左侧是 l_value 的硬性要求，可有如下代码：

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


[1]:https://craftinginterpreters.com/
[2]:https://www.youtube.com/playlist?list=PLj_VrUwyDuXS4K3n7X4U4qmkjpuA8rJ76