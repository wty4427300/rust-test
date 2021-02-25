use std::thread;
//跑一个线程,保证自己的在main结束之前执行完毕
fn test_1(){
    let handle=thread::spawn(||{
        for i in 1..10{
            println!("{}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    //等待其他线程执行完毕
    handle.join().unwrap();
}
//闭包捕获住线程的变量的
fn test_2(){
    let v=vec![1,2,3];
    //所有权属于子线程,住线程不能再drop(v)
    let handle =thread::spawn(move ||{
        println!("Here's a vector: {:?}",v)
    });

    handle.join().unwrap();
}