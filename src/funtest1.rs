//方法的语法
struct RecTangle{
    width: u32,
    height: u32,
}

impl RecTangle{
    fn area(&self)-> u32{
        self.width*self.height
    }
}

fn fun_test1(){
    let rec =RecTangle{width:30,height:50};
    println!("{}",rec.area());
}