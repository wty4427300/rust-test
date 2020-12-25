//定义一个泛型结构体,且下x,y是同一个类型
struct Point<T> {
    x: T,
    y: T,
}

//定义一个泛型结构体,且下x,y是不同类型
struct Point1<T, U> {
    x: T,
    y: U,
}

//枚举定义泛型
enum Option<T> {
    Some(T),
    None,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T, U> Point1<T, U> {
    fn mixUp<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

fn trait_test() {
    let test = vec![10, 20, 30, 40, 66];
    // let result=largest_1(&test);
    // println!("The largest number is {}", result);
    //统一类型
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//需要声明这个函数是一个泛形函数，返回值是一个泛形，参数也是一个泛形
//报错的原因的是类型信息不足，或者是没有实现该类型的比较逻辑
//参数类是可比较可以copy的
fn largest_1<T:PartialOrd+Copy>(list:&T)->T{
    let mut largest=list[0];

    for &item in list.iter(){
        if item>largest{
            largest=item;
        }
    }
    largest
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    //默认的实现可以调用同一个trait中的其他实现
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify1<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

//这样写可以省略impl Summary
pub fn notify2<T: Summary>(item1: T, item2: T) {}

pub trait Display {}

//多trait
pub fn notify3(item: impl Summary + Display) {}

//多trait
pub fn notify4<T: Summary + Display>(item: T) {}

//不同泛型参数，且多trait的情况
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

//用where来简化上面这种写法
fn some_function１<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}

//返回值也是一个实现了trait的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

