//定义一个泛型结构体,且下x,y是同一个类型
struct Point<T>{
    x:T,
    y:T,
}

//定义一个泛型结构体,且下x,y是不同类型
struct Point1<T,U>{
    x:T,
    y:U,
}

//枚举定义泛型
enum Option<T>{
    Some(T),
    None,
}

impl <T> Point<T>{
    fn x(&self) -> &T{
        &self.x
    }
}

impl<T,U> Point1<T,U>{
    fn mixUp<V,W>(self,other:Point1<V,W>)->Point1<T,W>{
        Point1{
            x:self.x,
            y:other.y,
        }
    }
}

fn trait_test(){
    let test=vec![10,20,30,40,66];
    let result=largest_1(&test);
    // println!("The largest number is {}", result);
    //统一类型
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

fn largest(list:&[i32])->i32{
    let mut largest =list[0];

    for &item in list{
        if item>largest {
            largest=item;
        }
    }
    largest
}

//需要声明这个函数是一个泛形函数，返回值是一个泛形，参数也是一个泛形
//报错的原因的是类型信息不足，或者是没有实现该类型的比较逻辑
fn largest_1<T>(list:&T)->T{
    let mut largest=list[0];

    for &item in list.iter(){
        if item>largest{
            largest=item;
        }
    }
    largest
}

pub trait Summary{
    fn summarize(&self) -> String;
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
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}





