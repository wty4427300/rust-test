use std::thread;

//跑一个线程,保证自己的在main结束之前执行完毕
fn test_1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{}", i);
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
fn test_2() {
    let v = vec![1, 2, 3];
    //所有权属于子线程,线程不能再drop(v)
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v)
    });

    handle.join().unwrap();
}

use std::sync::mpsc;

//mpsc是多个生产者,一个消费者的简写
//send还会获取所有权，所以send后就别再在当前线程使用了
fn test_3() {
    //通道用来实现生产者，消费者模式
    let (tx, rx) = mpsc::channel();

    thread::spawn(move||{
       let val=String::from("hi");
        //将数据放入通道，和go的并无差别，异常时调用unwrap()会抛出一个panic
        tx.send(val).unwrap();
    });

    let received =rx.recv().unwrap();
    println!("got:{}",received);
}

fn test_4(){
    let (tx,rx)=mpsc::channel();

    thread::spawn(move||{
        let vals=vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recv in rx {
        println!("got:{}",recv)
    }
}


