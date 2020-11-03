//简单的trait
struct A;

impl A{
    fn hello(&self){
        println!("in A")
    }
}

//也就是会说继承Hello这个trait必须实现hello这个函数
trait  Hello{
    fn hello(&self);
}


impl Hello for A{
    fn hello(&self) {
        println!("from hello trait");
    }
}
fn test(){
    let a=A;
    a.hello();
    //执行继承了trait Hello的hello方法
    //所谓的歧义语句
    <A as Hello>::hello(&a);
}