![Static Badge](https://img.shields.io/badge/Ubuntu-True-blue)
![Static Badge](https://img.shields.io/badge/Windows-False-red)
![Static Badge](https://img.shields.io/badge/Language-Rust-purple)
![Static Badge](https://img.shields.io/badge/For-Novice-brown)
---
#### 使用rust👾实现一个compiler💻.
---

#### 代码逻辑来自[Crafting-Interpreters][1] ( 原代码为java实现 ) ;

#### 代码实现参考 Youtube上的 [rust 实现版本][2]。
---
### Content:

1. #### Scanner : Scan the source code.

   String => Vec[Token]

   不断的扫描当前字符，或peek下一个字符，来判断当前的这个word是哪一个token。
   
   ![scan_tokens](https://github.com/superbignut/ltl-compiler/blob/master/sources/scan_tokens.png)

2. #### Representing Code : A representation for code.
   
   AST => String

   通过递归调用 two_string 将 Expr 转换为 String，相当于手动 ‘反向’ 实现了下一节的内容。


   ![represent_code](https://github.com/superbignut/ltl-compiler/blob/master/sources/represent_code.png)




[1]:https://craftinginterpreters.com/
[2]:https://www.youtube.com/playlist?list=PLj_VrUwyDuXS4K3n7X4U4qmkjpuA8rJ76