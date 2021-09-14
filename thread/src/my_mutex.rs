
use std::sync::{Mutex, Arc};
use std::thread;

///  新建互斥锁
pub fn mutex_new(){
    let num = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let num = Arc::clone(&num);
        let handle = thread::spawn(move || {
            let mut a = num.lock().unwrap();
            *a += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", *num.lock().unwrap());


}