//sum之后不允许使用iter,因为sum获取iter的所有权
fn test(){
    let v1=vec![1,2,3,];

    let v1_iter=v1.iter();

    let total:i32=v1_iter.sum();

    println!("sum {}",total);
}

fn test_1(){
    let v1=vec![1,2,3,];

    let v2:vec<_>=v1.iter().map(|x|x+1).collect();

}

struct Shoe{
    size:u32,
    style:String,
}

fn shoes_in_my_size(shoes:Vec<Shoe>,shoe_size:u32)->Vec<Shoe>{
    shoes.into_iter()
        .filter(|s|s.size==shoe_size)
        .collect()
}

struct Counter{
    count:u32,
}

impl Counter{
    fn new()->Counter{
        Counter{
            count:0
        }
    }
}
//自定义的iter,只存储5个数
impl Iterator for Counter{
    //返回值类型
    type Item=i32;
    fn next(&mut self)->Option<self::Item>{
        self.count+=1;

        if self.count<6 {
            Some(self.count)
        }else {
            None
        }
    }
}