#![allow(warnings)]
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::vec;
use std::error::Error;

fn run_file(file_path: &str) -> Result<(), io::Error> {
    let content = fs::read_to_string(Path::new(file_path))?;

    println!("File: '{}' was read successfully!", file_path);

    return Result::Ok(()); // Result::Err()
}

fn run() {}

// *单词编码
enum TokenCode {
    //运算符、分隔符
    TkPlus,  //+
    TkMinus, //-
    TkStar,  //*
    TkDivide,
    TkMod,       //%
    TkEq,        //==
    TkNeq,       //！=
    TkLt,        //<
    TkLeq,       //<=
    TkGt,        //>
    TkGeq,       //>=
    TkAssign,    //=
    TkPointsto,  //->
    TkDot,       //.
    TkAnd,       //&
    TkOpenpa,    //(
    TkClosepa,   //)
    TkOpenba,    //[
    TkClosebr,   //]
    TkBegin,     //{
    TkEnd,       //}
    TkSemicolon, //;
    TkComma,     //,
    TkEllipsis,  //...
    TkEof,       //文件结束符
    //关键字
    KwChar,     //char
    KwInt,      //int
    KwShort,    //short
    KwVoid,     //void
    KwStruct,   //struct
    KwIf,       //if
    KwElse,     //else
    KwFor,      //for
    KwContinue, //continue
    KwBreak,    //break
    KwReturn,   //return
    KwSizeof,   //sizeof
    KwAllign,   //__align
    KwCdecl,    //__cdecl
    KwStdcall,  //__stdcall

    //常量
    TkCint,  //整形常量
    TkCchar, //字符常量
    TkCstr,  //字符串
}

// *单词存储结构
struct TokenWord {
    tkcode: TokenCode,            //单词编码
    next: Option<Box<TokenWord>>, //指向同义词,但有可能是空
    spelling: String,             //拼写
                                  //sym_struct
                                  //sym_identifier
}

// *哈希表容量
const MAX_KEY: usize = 1024;

// *单词表存储结构
struct TokenTable {
    tk_table: Vec<TokenWord>,
    tk_hash_table: [Option<Box<TokenWord>>; MAX_KEY],
}
// *TokenTable的成员函数
impl TokenTable {
    // ?关键字插入单词表函数
    fn token_word_direct_insert(self: &mut Self, token: TokenWord) {
        self.tk_table.push(token);

        let key_number = 1;
    }

    // ?在单词表中查找单词
    fn token_word_find(self: &Self, find_word: &str) {}

    // ?单词标插入单词，会先查找
    fn tocken_word_insert(self: &Self, find_word: &str) {}

    // ? 初始化单词表
    fn new() -> TokenTable {
        let keywords = vec![
            TokenWord {
                tkcode: TokenCode::TkPlus,
                next: None,
                spelling: String::from("+"),
            },
            TokenWord {
                tkcode: TokenCode::TkMinus,
                next: None,
                spelling: String::from("-"),
            },
            TokenWord {
                tkcode: TokenCode::TkStar,
                next: None,
                spelling: String::from("*"),
            },
            TokenWord {
                tkcode: TokenCode::TkDivide,
                next: None,
                spelling: String::from("/"),
            },
        ];

        const INIT: Option<Box<TokenWord>> = None;
        let hash_table: [Option<Box<TokenWord>>; 1024] = [INIT; 1024];

        TokenTable {
            tk_table: keywords,
            tk_hash_table: hash_table,
        }
    }

    // ? 计算hash值的函数
    fn compute_elf_hash(input: &String) -> u32 {
        let mut hash: u32 = 0;
        for byte in input.bytes() {
            hash = (hash << 4) + u32::from(byte);
            let x = hash & 0xF0000000;
            if x != 0 {
                hash ^= (x >> 24);
            }
            hash &= (!x);
        }
        return hash % MAX_KEY as u32;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let token_table = TokenTable::new(); //初始化单词表

    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            println!("File: '{}' is going to be compiled! ", &args[1]);
            Ok(())
            // if let Err(err) = run_file(&args[1]) {
            //     println!("Error occured in readfile: {}", &args[1]);
            // }
        }
        1 => {
            // * 没有输入额外参数，读取默认文件sources/test.cpp
            println!("No parameters input, compile default file: test.cpp.");

            let err = fs::read("sources/test.cpp");
            match err {
                Err(e) => {
                    println!("Error occured in reading sources/test.cpp");
                    Ok(())
                }
                Ok(fd) => {
                    println!("{:?},{}", fd, fd.len());
                    
                    Ok(())
                }
            }
        }
        _ => {
            println!("Error!");
            Ok(())
        }
    }



}
