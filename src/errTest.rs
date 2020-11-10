use std::fs::File;
use std::io::ErrorKind;

enum Result<T,E>{
    Ok(T),
    Err(E),
}


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