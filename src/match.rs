enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Pr,
}

fn value_in_cents() -> u8 {
    match coin {
        Coin::Dime=>10,
        Coin::Nickel=>9,
        Coin::Penny=>8,
        Coin::Quarter=>7,
        Coin::Pr=>{
            printli!("luck!");
        }
    }
}