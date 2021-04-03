pub struct Post{
    state:Option<Box<dyn State>>,
    content:String,
}

trait State{
    fn request_review(self:Box<self>)->Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}
struct Draft{}
impl State for Draft{
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
struct PendingReview {}
struct Published {}

impl State for Published{
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    //因为我返回的是post的一部分，所和post要在一个生命周期内
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

impl State for PendingReview{
    fn request_review(self:Box<self>)->Box<dyn State>{
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

//博文的初始化,状态和文章都是初始化。
impl Post{
    pub fn new()->Post{
        Post{
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    //获取可变引用是因为，我们正要改变post实例的属性
    pub fn add_text(&mut self,text:&str){
        self.content.push_str(text)
    }

    //as_ref是获取引用
    pub fn content(&self)->&str{
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self){
        //从box里面拿出state然后消费state并返回一个新的state
        if let Some(s)=self.state.take(){
            self.state=Some(s.request_review())
        }
    }

    pub fn approve(&mut self){
        if let Some(s)=self.state.take(){
            self.state=Some(s.approve())
        }
    }
}

