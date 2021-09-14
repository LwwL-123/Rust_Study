use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// 新建一个线程
pub fn thread_new(){
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread {}", i);
        }
    });
    handle.join().unwrap();

    for i in 0..20 {
        print!("{}",i);
    }
}

/// 使用管道传输信息
pub fn mpsc_new1(){
    let (tx,rx) = mpsc::channel();
    thread::spawn(move || {
        let string = String::from("new a channel");
        tx.send(string).unwrap();
    });

    let string1 = rx.recv().unwrap();
    println!("接收到：{}",string1)
}

/// 单个生产者 单个消费者
pub fn mpsc_new2(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
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

    for received in rx {
        println!("Got: {}", received);
    }
}

/// 多个生产者 单个消费者mpsc
pub fn mpsc_new3(){
    let (tx,rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    thread::spawn(move||{
        let vec1 = vec![
            String::from("tx1"),
            String::from("tx11"),
            String::from("tx111"),
        ];
        for value in vec1 {
            tx1.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move||{
        let vec2 = vec![
            String::from("tx2"),
            String::from("tx22"),
            String::from("tx222"),
        ];
        for value in vec2 {
            tx2.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

}