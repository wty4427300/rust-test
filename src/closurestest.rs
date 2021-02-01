use std::*;

//用于存放闭包的结构体
struct Cacher<T> where T:Fn(u32)->u32
{
        calculation: T,
        value: Option<u32>,
}

impl <T> Cacher<T> where T:Fn(u32)->u32{
    //初始化
    fn new(calculation:T)->Cacher<T>{
        Cacher{
            calculation,
            value:None,
        }
    }
    //如果some有值则返回值，没有执行闭包
    fn value(&mut self,arg:u32)->u32{
        match self.value{
            Some(v)=>v,
            None=>{
                let v=(self.calculation)(arg);
                self.value=Some(v);
                v
            },
        }
    }
}

//以后每个rs文件中的test作为一个模块单独main函数
fn test(){
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    //闭包参数为num,如果多于一个则用逗号隔开
    let expensive_closure=|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    generate_workout_1(simulated_user_specified_value, simulated_random_number);
}

//根据数值的不同有不同的打印
fn generate_workout_1(intensity:u32,random_number:u32){
    if intensity<32 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    }else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

//如过intensity大于25且random_number不等于3这话死后就不需要执行
//simulated_expensive_calculation,此时如果我们把expensive_result
//定义成一个闭包,我们就可以省略不必要的代码运行,只在需要运行的时候运行它
fn generate_workout(intensity:u32,random_number:u32){
    //闭包参数为num,如果多于一个则用逗号隔开
    let expensive_closure=|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }

}

fn generate_workout_2(intensity:u32,random_number:u32){
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

//打印参数并阻塞两秒
fn test1(){
    fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    }
}

//省略了很多代码,主要就是缓存闭包的结果,减少闭包的执行次数
fn generate_workout_4(intensity:u32,random_number:u32){
    let mut result=Cacher::new(|num|{
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity<25 {
        result.value(intensity);
        result.value(intensity);
    }else {
        if random_number==3 {
            result.value(intensity);
        }else {
            result.value(intensity);
        }
    }
}





