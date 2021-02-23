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

struct CustomSmartPointer{
    data:String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn test_5(){
    let c=CustomSmartPointer{data:String::from("hanhan")};
    let d=CustomSmartPointer{data:String::from("hanpi")};
    println!("CustomSmartPointers created.");
}

//主动调用drop（std::mem::drop）
fn test_6(){
    let c=CustomSmartPointer{data:String::from("some data")};
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}


enum MyList{
   Conss(i32,Rc<MyList>),
   Null,
}
//Rc共享所有权
fn test_7(){
    let a=Rc::new(Conss(5,Rc::new(Conss(10,Rc::new(Null)))));
    let b=Conss(3,Rc::clone(&a));
    let c=Conss(4,Rc::clone(&a));
}
//Rc::clone不会真的clone只会增加引用计数器,使用与不可变引用
fn test_8() {
    let a = Rc::new(Conss(5, Rc::new(Conss(10, Rc::new(Null)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Conss(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    //进入该作用域计数器加1,出作用域减1
    {
        let c = Conss(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

pub trait Messenger{
    fn send(&self,msg:&str);
}

pub struct LimitTracker<'a,T:Messenger>{
    messenger:&'a T,
    value:usize,
    max:usize,
}

impl <'a,T>LimitTracker<'a,T> where T:Messenger{
    pub fn new(messenger:&T,max:usize)->LimitTracker<T>{
        LimitTracker{
            messenger,
            value:0,
            max,
        }
    }

    pub fn set_value(&mut self,value:usize){
        let percentage_of_max =self.value as f64 /self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

struct MockMessenger{
    sent_messages:RefCell<Vec<Sting>>,
}

impl MockMessenger{
    fn nwe()->MockMessenger{
        MockMessenger{
            sent_messages:RefCell::new(vec![])
        }
    }
}
//虽然传入的是不可变的self,但是sent_messages是Refcell保存的，可以获取方法内部的可变性，可以进行修改
impl Messenger for MockMessenger{
    fn send(&self, msg: &str) {
        self.sent_messages.borrow_mut().push(String::from(msg));
    }
}

enum NewList{
    NewCons(Rc<RefCell<i32>>,Rc<NewList>),
    NewNil,
}

fn test_10(){
    let value=Rc::new(RefCell::new(5));

    let a=Rc::new(NewCons(Rc::clone(&value),Rc::new(NewNil)));

    let b=NewCons(Rc::new(RefCell::new(6)),Rc::clone(&a));

    let c=NewCons(Rc::new(RefCell::new(10)),Rc::clone(&a));

    *value.borrow_mut() += 10;
}









