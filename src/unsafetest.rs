use std::slice;
use std::ops::Add;

fn test(){
    let mut num=5;
    //不可变裸指针
    let r1=&num as *const i32;
    //可变裸指针
    let r2=&mut num as *mut i32;
    let address = 0x012345usize;
    let r = address as *const i32;
    //裸指针只能在unasfe代码块里面解引用
    unsafe{
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        test2()
    }
}

unsafe fn test2(){}

// fn test3(){
//     let mut v=vec![1,2,3,4,5,6];
//     let r =&mut v[..];
//     let(a,b)=slip_at_mut(r, 3);
//     assert_eq!(a, &mut [1, 2, 3]);
//     assert_eq!(b, &mut [4, 5, 6]);
// }

//同一个切片借用了两次不安全，所以想要使用需要加unsafe代码块
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        //开头到中间
        (slice::from_raw_parts_mut(ptr, mid),
         //中间到结尾
         slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}

// static hellow:&str="hanhan";
// //读写静态变量是不安全的，需要unsafe,
// fn test4(){
//     println!("{}",hellow);
//     unsafe {
//         hellow+"sssss";
//     }
// }

//不安全的trait
unsafe trait Foo{

}

unsafe impl Foo for i32{

}
//运算符重载实现坐标相加
#[derive(Debug, PartialEq)]
struct Point{
    x:i32,
    y:i32,
}
impl Add for Point{
    type Output = Point;

    fn add(self, other:Point) -> Point {
        Point{
            x:self.x+other.x,
            y:self.y+other.y,
        }
    }
}

fn test5() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}

struct Millimeters(u32);
struct Meters(u32);

//不使用默认参数类型，
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}


use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct PointOut {
    x: i32,
    y: i32,
}

impl fmt::Display for PointOut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for PointOut {

}

struct  Wrapper(Vec<String>);
//format ，被格式化的样子 ，以及什么符号做间隔
impl fmt::Display for Wrapper{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"[{}]",self.0.join(","))
    }
}

fn test6() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

type Kilometers =i32;
fn test7(){
    let x:i32=8;
    let y:Kilometers=15;
    let z=x+y;
    println!("{}",z);
}
//!没返回值
fn test8()-> ! {
    let x=1;
}
fn add_one(x:i32)->i32{
    x+1
}

fn do_write(f:fn(i32)->i32,arg:i32)->i32{
    f(arg)+f(arg)
}

//朋友被我拉入坑了，送他一个hello word
fn test9(){
    let s="我是你爹";
    println!("{}",s);
}

fn test10(){
    let list_of_numbers=vec![1,2,3];
    let list_of_string:Vec<Sting> =list_of_numbers.iter().map(|i|i.to_string()).collect();
}

enum  Status{
    Value(u32),
    Stop,
}

fn test11(){
    let list_of_numbers:Vec<Status>=(0u32..20).map(Status::Value).collect();
}

