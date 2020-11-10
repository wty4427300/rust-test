use std::collections::HashMap;

fn test(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.insert(String::from("Blue"), 25);
    //同名的key新的的value会覆盖老的value

    scores.entry(String::from("Blue")).or_insert(50);
    //key有没有关联一个值没有的话,插入value 50

    let text ="hello world wonderful world";

    let mut map =HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        //返回的是value的引用,所以需要用*解引用
        *count+=1;
    }
    println!("{:?}", map);

    for (key,value) in &scores {
        println!("{}: {}", key, value);
    }

    let teams =vec![String::from("Blue"),String::from("Yellow")];
    let initial_scores = vec![10, 50];


    //zip用来创建一个元组的vec,两个iter创建好了,之后调用collect转换成一个map
    let score:HashMap<_,_> =teams.iter().zip(initial_scores.iter()).collect();

    let field_name =String::from("hanHan");
    let field_value = String::from("shaBi");

    let mut map=HashMap::new();
    map.insert(field_name,field_value);
    //插入之后这两string的所有权就属于map了,在使用就会报错.毕竟不是基础类型不能copy

    let mut score =HashMap::new();
    score.insert(String::from("hanHan"),10);
    score.insert(String::from("hanPi"),10);

    let team_name=String::from("Blue");
    let score_t =score.get(&team_name);

}