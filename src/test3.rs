#[derive(Debug)]
enum Gender{
    Unspecified=0,
    Females=1,
    Male=2,
}

#[derive(Debug,Copy,Clone)]
struct UserId(u64);

#[derive(Debug,Copy,Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User{
    id:UserId,
    name:String,
    gender:Gender,
}

//定义聊天室中可以发生的事
#[derive(Debug)]
enum Event{
    Join((UserId,TopicId)),
    Leave((UserId,TopicId)),
    Message((UserId,TopicId,String)),
}

fn test(){
    let alice=User{
        id:UserId(1),
        name:"wty".into(),
        gender:Gender::Female
    };
}