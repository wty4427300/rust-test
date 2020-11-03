//闭包
fn test3(){
    //未捕捉环境变量
    let c1=||println!("hello");
    c1();

    //可修改环境变量
    let mut arr =[1,2,3];
    let mut c2=|i|{
        arr[0]=i;
        println!("{:?}",arr);
    };
    c2(0);

    //未修改环境变量
    let anwser=42;
    let c3=||{
        println!("{}",anwser)
    };
    c3();

    let a=42;
    let ref b=a;
    let c=&a;
}
