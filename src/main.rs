// use std::cell::RefCell;
// use std::rc::{Rc, Weak};
// use crate::List::{Cons, Nil};
// use std::option::Option::Some;
use std::time::Duration;
// use crate::List::{Cons, Nil};
// use std::ops::Deref;
// use std::alloc::handle_alloc_error;
// use std::rc::Rc;
// use std::slice::RChunks;
// use crate::MyList::{Conss, Null};
// use std::cell::RefCell;
// use crate::NewList::{NewCons, NewNil};
// use std::borrow::Borrow;
//use std::fmt::{Debug, Display};
// include!("test.rs");
// include!("funtest.rs");
// include!("rust-test.rs");
// include!("funtest1.rs");
// include!("MyPointer.rs");
// include!("vectest.rs");
// include!("strtest.rs");
//include!("traitTest.rs");
//include!("iotest.rs");
//include!("closurestest.rs");
// include!("itertest.rs");
// include!("autoptr.rs");
// mod errTest;
// mod MyPointer;
// use std::{env, fs, process};
// use std::error::Error;
// use std::time::Duration;
// use std::panic::resume_unwind;
// include!("ptrtest.rs");
include!("threadtest.rs");


fn main() {
    test_4();
    // test_1();
    //test_8();
    // //获取命令行参数并打印
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // let config =Config::new(&args).unwrap_or_else(|err|{
    //     //标准错误
    //     eprintln!("输出错误信息: {}", err);
    //     //非0的错误码,代表非正常退出
    //     process::exit(1);
    // });
    // if let Err(e)=io_test(config){
    //     println!("Application error: {}", e);
    //     process::exit(1);
    // };
    //
    // let x:Option<i32>=None;
    // for number in (1..10).rev() {
    //     println!("{}!", number);
    // }
    // println!("LIFTOFF!!!");
    //
    // let _x=5;
    //
    // let x=6;
    //
    // let f: bool =true;
    //
    // if f {
    //     println!("憨憨");
    // }
    //
    // println!("{}",x);
    // //元组
    // let tup:(i32,f64,u8)=(500,3.14,1);
    // println!("{},{},{}",tup.0,tup.1,tup.2);
    // //数组
    // let _array:[i32;5]=[1,2,3,4,5];
    // another(10,15);
    // println!("{}",five());
    // let number=10;
    // if number<10 {
    //     println!("麻瓜")
    // }else {
    //     println!("红薯")
    // }
    //
    // let mut fun=counter(10);
    // let result=fun(5);
    // println!("{}",result);
    //
    // test();
    // fun_test();
    // test3()
    // fun_test1();
    // let a=my_pointer();
    // println!("{}",*a);
    // test();
    //trait_test()
}

// fn another(x:i32,y:i32){
//     println!("开始执行我的函数:{},{}",x,y);
// }
//
// fn five()->i32{
//     5
// }
//
// fn counter(i:i32)-> impl FnMut(i32)->i32 {
//     move |n| n + i
// }
