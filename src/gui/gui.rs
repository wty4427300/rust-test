pub trait Draw{
    fn draw(&self);
}

//暂时理解为box里存储的实现了draw trait的实例
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

//这个方法就是迭代vec里面的对象并执行
impl Screen{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw()
        }
    }
}

pub struct Button{
    pub width: u32,
    pub height: u32,
    pub label:String,
}

impl Draw for Button{
    fn draw(&self) {

    }
}

struct SelectBox{
    width:u32,
    height:u32,
    options:Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self) {

    }
}
//感觉来了，不同类型统一的执行流程
fn test1(){
    let screen =Screen{
        components: vec![
            Box::new(SelectBox{
                width: 0,
                height: 0,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ]
            }),
            Box::new(Button{
                width: 0,
                height: 0,
                label: String::from("ok"),
            })
        ]
    };

    screen.run();
}



