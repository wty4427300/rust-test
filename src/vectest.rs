fn test(){
    let mut v:Vec<i32>=Vec::new();
    let v1 = vec![1, 2, 3];
    println!("{}",v1[2]);
    v.push(5);
    v.push(4);
    v.push(3);
    v.push(2);

    let x=&v[2];
    println!("{}",x);

    match v.get(0){
        Some(v) => println!("{}",v),
        None=>println!("没有匹配到")
    }
}