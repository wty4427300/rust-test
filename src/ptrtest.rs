#[derive(Debug)]
enum List{
    Cons(i32,RefCell<Rc<List>>),
    Nil,
}

impl List{
    fn tail(&self)-> Option<&RefCell<Rc<List>>>{
        match self {
            Cons(_,item)=>Some(item),
            Nil=>None,
        }
    }
}
//没有debug标签,不让format List
//循环引用一哈
fn test_1(){
    let a =Rc::new(Cons(5,RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b=Rc::new(Cons(10,RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link)=a.tail(){
        *link.borrow_mut()=Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
}
#[derive(Debug)]
struct Node{
    value:i32,
    //父节点
    parent:RefCell<Weak<Node>>,
    //子节点
    children:RefCell<Vec<Rc<Node>>>,
}

fn test_11(){
    //初始化节点
    let leaf =Rc::new(Node{
        value:3,
        parent:RefCell::new(Weak::new()),
        children:RefCell::new(vec![]),
    });
    //打印父节点
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch =Rc::new(Node{
        value:5,
        parent:RefCell::new(Weak::new()),
        children:RefCell::new(vec![Rc::clone(&leaf)]),
    });
    //设置父节点downgrade 弱引用加1
    *leaf.parent.borrow_mut()=Rc::downgrade(&branch);

    //打印父节点
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
