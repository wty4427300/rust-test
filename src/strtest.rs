fn test(){
    // let  s=String::new();

    let data = "initial contents";

    let i=data.to_string();

    println!("{}",i);

    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    let mut hello = String::from("Hola");

    hello.push_str("-insert");

    println!("{}",hello);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}