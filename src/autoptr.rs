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
//一个简单的解引用过程
fn test_2(){
    let x=5;
    let y=&x;
    let c=*y;
    println!("{}",c);
}
//使用box<T>
fn test_3(){
    let x=5;
    let y=Box::new(x);
    let c=*y;
    //*(y.deref())这一句在编译器底层相当于调用deref方法
    println!("{}",c);
}
struct  MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}
//返回引用而不是值的原因在于所有权,此时所有权仍然在self,如果返回值本身的话,所有权就要移出self
impl <T> Deref for MyBox<T>{
    //这一句是定义trait的关联类型
    type Target=T;
    fn deref(&self)->&T {
        &self.0
    }
}
//解引用强制多态
fn test_4(){
   let m=MyBox::new(String::from("Rust"));
    //将&string转换成&str
   hello(&m);
}

fn hello(name:&str){
    println!("hello,{}!",name);
}








