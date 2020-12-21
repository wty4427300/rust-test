// use std::ops::Deref;
// struct MyPointer<T>(T);
//
// //只能指针的new方法
// impl <T> MyPointer<T>{
//     fn new(x:T)->MyPointer<T>{
//         MyPointer(x)
//     }
// }
//
// impl<T> Deref for MyPointer<T>{
//     type Target = T;
//     fn deref(&self)->&T{
//         &self.0
//     }
// }
//
// fn my_pointer()-> MyPointer<i32>{
//     let x=5;
//     let y = MyPointer::new(x);
//     return y;
// }