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
   
   Expr(AST) => String

   通过递归调用 two_string 将 Expr 表达式 或者说一个简单的抽象语法树 AST 转换为 String，相当于手动 ‘反向’ 实现了下一节的内容 ;


   ![represent_code](https://github.com/superbignut/ltl-compiler/blob/master/sources/represent_code.png)


3. #### Parsing Expressions

   Vec[Token] => Expr

   使用递归下降法，逐步将一组 Token 匹配成一个表达式 Expr ; 具体匹配规则如下，越向下优先级越高：

         最顶层-表达式： expression -> equality

         等式表达式： equality -> comparision ( ("!=" | "==") comparision  ) * ;

         不等表达式： comparision -> term ( ( ">" | ">=" | "<" | "<=") ) * ;

         加法表达式： term -> factor ( ( "-" | "+" ) factor ) * ;

         乘法表达式： factor -> unary ( ( "/" | "*") unary ) * ;

         一元表达式： unary -> ( ( "!" | "-" ) unary ) | primary ;
         
         最底层-基础单元： primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
   
   并且，这里的规则实现，刻意的避免了前缀表达式的写法 ; 递归下降法的代码实现十分巧妙,部分如下:

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



[1]:https://craftinginterpreters.com/
[2]:https://www.youtube.com/playlist?list=PLj_VrUwyDuXS4K3n7X4U4qmkjpuA8rJ76