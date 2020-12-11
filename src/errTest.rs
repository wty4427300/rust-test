use std::io::{ErrorKind, Read};
use std::io::Error;
use std::io;
use std::fs::File;
use std::result;

// enum Result<T,E>{
//     Ok(T),
//     Err(E),
// }


fn test(){
    // panic!("造成了恐慌")
    let f=File::open("hello.txt");
    //成功返回文件句柄,失败返回错误信息
    let f=match f {
        Ok(file)=>file,
        Err(error)=>{
            panic!("Problem opening the file: {:?}", error)
        },
    };
}

//详细一点的错误处理
fn test1(){
    let f=File::open("shabi.txt");

    let f=match f{
        Ok(file)=>file,
        Err(error)=>match error.kind(){
            //没有文件的话,就创建文件并返回文件句柄
            ErrorKind::NotFound=>match File::create("hello.txt"){
                Ok(fc)=>fc,
                Err(e)=>panic!("Problem creating the file: {:?}", e),
            }
            other_error=>panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

//比包捕获error
fn test2(){
    let f =File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind()==ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("Problem creating the file: {:?}", error);
            })
        }else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

//unwrap对Result<T>做了match匹配
fn test3(){
    let f = File::open("hello.txt").unwrap();
}

//unwrap和expect的区别就好似可以自定义错误信息
fn test4(){
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

//抛出异常
fn read_username_from() -> std::result::Result<String, Error> {
    let f =File::open("hello.txt");

    let mut f=match f {
        Ok(file)=>file,
        Err(e)=>return Err(e),
    };

    let mut s=String::new();

    match f.read_to_string(&mut s) {
        Ok(_)=>Ok(s),
        Err(e)=>Err(e),
    }
}

//panic的简写unwrap和expect
//默认错误信息和自定义错误信息

fn err_easy(){
    let f=File::open("hello.txt").unwrap();
    let file=File::open("hello.txt").expect("文件打开失败");
}

fn read_username_from_file() -> std::result::Result<String, Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

//传播错误的简写(?),也就是说?和match是差不多的
fn err_easy_two()-> Result<String, io::Error>{
    // let mut f=File::open("hello.txt");
    let mut s =String::new();
    // f.read_to_string(&mut s)?;
    // 压缩一下代码
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}



fn test5(){
    let mut f =File::open("hello.txt");
    let mut s = String::new();
}

//最后废话几句
//使用panic就本就默认粗错误不可恢复了,使用result的话就是吧错误丢给调用者去处理
//也就是说写业务代码用panic,写工具或者库用result




