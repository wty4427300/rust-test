//高贵的一等公民函数

type RGB=(i16,i16,i16);

fn color(_c:&str)->RGB{
    (1,1,1)
}
//函数可以作为参数
fn show(c:fn(&str)->RGB){
    println!("{:?}",c("block"));
}

fn fun_test(){
    //此时rgb是一个函数项类型，函数项是0大小的
    let rgb=color;
    //这里隐式的把函数项类型转换为函数指针类型
    show(rgb);
    //定义了一个实现了Fn(&str)->RGB trait的闭包类型
    let c=|s:&str|{
        (2,3,4)
    };;
    show(c)
}
