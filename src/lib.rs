//尝试测试另外一个模块
mod iotesttdd;


pub fn add_two(a:i32) ->i32{
  a+2
}

pub fn greeting(name:&str)->String{
    format!("Hello {}!",name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{add_two, greeting};

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    #[test]
    //表明这是一个测试函数
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
    //先写函数在写test注解要不然代码警告很烦

    #[test]
    fn larger_can_hold_smaller(){
        let larger=Rectangle{width:8,height:9};
        let smaller=Rectangle{width:5,height:1};
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn si_adds_two(){
        assert_eq!(4,add_two(2));
    }
    //自定义错误信息
    #[test]
    fn greeting_contains_name(){
        let result=greeting("傻逼");
        assert!(result.contains("傻逼"),"错误");
    }
    //result用于测试
    #[test]
    fn is_works()->Result<(),String>{
        if 2+2==4 {
            Ok(())
        }else {
            Err(String::from("错了"))
        }
    }

    #[test]
    fn case_sensitive(){
        let query ="duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    }

    assert_eq!(
        vec!["safe, fast, productive."],
        //search已经改没了，写这个就是练一下代码风格
        search(query, contents)
    );


}
//简单的判断了一下2+2=4