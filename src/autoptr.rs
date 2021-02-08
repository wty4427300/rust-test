//离开作用于自动释放
fn test(){
    let b=Box::new(5);
    println!("b={}",b)
}

enum List{
    Cons(i32,Box<List>),
    Nil,
}

//因为递归在编译期无法计算大小
fn test_1(){
    let list=Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));
}


