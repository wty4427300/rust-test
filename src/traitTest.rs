fn trait_test(){
    let test=vec![10,20,30,40,66];
    let result=largest(&test);
    println!("The largest number is {}", result);
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