fn test(){
    //创建了一个i32的可变数组
    let mut arr = vec![1, 2, 3];
    // cache the last item,这里修改方式应该是获取只读的借用,这样所有权不变,arr就可以继续写
    let last = arr.last();
    // consume previously stored last item
    println!("last: {:?}", last);
    arr.push(4);
}