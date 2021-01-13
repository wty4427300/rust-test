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
fn largest_1<T: PartialOrd + Copy>(list: &T) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
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

//PartialOrd是一个标准库的trait包含大于小于的计算
//复习一个概念基础数据类型一般都是已知大小的，所以可以他们实现了copy trait
//现在我们将参数改为泛型后就没有实现copy trait了所以不能copy
//这就是一个泛型比较大小的函数
fn largest_2<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

//之前报错就是因为引用的当前自定义的display
//只有标准库的display才可以打印
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x < self.y {
            println!("y = {}", self.y);
        } else {
            println!("x = {}", self.x);
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn test_3() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("");
        let result = longest(string1.as_str(), string2.as_str());
        println!("thr longest string is {}", result);
    }
}

//这里会报错，因为longest函数的生命周期设定是和两个参数中较短的那个生命周期保持一致也就是string2保持一致
//所以result的生命周期在出那个较小的作用域的时候就是已经结束了。
fn test_4() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

//一般情况下我们都会把返回值的的生命周期和一个参数做一个匹配，如果不和参数绑定的话就说明适合函数内部创建的值
//匹配了，就变成了一个悬垂引用。此时最好的方法就是返回一个有所所权的数据而不是返回一个引用。
fn test_5<'a>(x:&'a str,y: &str)->&'a str{
    x
}

//唠叨了半天，生命周期其实就干了一件事，将函数参数和返回值做一个绑定

//接下来是在结构体中注明生命周期
//因为我们将结构体中的引用变量注明了生命周期，所以该结构体的实例不能比part字段中的引用存在更久
struct ImportantExcerpt<'a>{
    part:&'a str,
}

fn test_6(){
    let novel=String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i=ImportantExcerpt{
        part:first_sentence
    };
}

//生命周期的省略
//不用写是因为几种特定的生命周期模式已经被写进编译器了
fn first_world(s:& str)->& str{
    let bytes=s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}