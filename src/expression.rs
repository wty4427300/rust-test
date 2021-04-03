fn test(){
    let favorite_color:Option<&str>=None;
    let is_tuesday =false;
    let age:Result<u8,_>="34".parse();

    if let Some(color)=favorite_color{
        println!("Using your favorite color, {}, as the background", color);
    }else if is_tuesday {
        println!("Tuesday is green day!");
    }else if let Ok(age)=age{
        println!("Using orange as the background color");
    }else {
        println!("Using blue as the background color");
    }
}

fn test1(){
    let mut stack =Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top)=stack.pop(){
        println!("{}",top)
    }
}

fn test2(){
    let v=vec!['a','b','c'];
    //打印索引和值
    for(index,value) in v.iter().enumerate(){
        println!("{} is at index {}", value, index);
    }
}

fn test3(){
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

fn test4(){
    let msg=Message::ChangeColor(0,160,255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
}

enum Message1 {
    Hello { id: i32 },
}

fn test5() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}


