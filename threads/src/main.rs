use std::thread;
use std::time::Duration;
use std::sync::{
    mpsc,
    Mutex,
    Arc,
};

fn main() {
    println!("base thread test started");
    let v = vec![1,2,3,4,5,6,7,8,9,10];

    let handle = thread::spawn(move || {
        for i in 0..v.len() {
            println!("handle thread vec: {}", v[i]);
        }
    });

    for i in 0..10 {
        println!("main thread i {}", i);
    };

    handle.join().unwrap();

    println!("channel test started");
    // multipe producer; single consumer
    let (tx1, rx) = mpsc::channel();

    let tx2 = tx1.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for rev in rx {
        println!("{}", rev);
    }

    println!("Mutex test started");
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    println!("Arc test started");

    let cnt = Arc::new(Mutex::new(0));
    let mut vec = vec![];

    for _ in 0..100 {
        let cnt = Arc::clone(&cnt);
        let handle = thread::spawn(move || {
            // 获取内部值的可变引用
            let mut num = cnt.lock().unwrap();
            *num += 1;
        });
        vec.push(handle);
    }

    for handle in vec {
        handle.join().unwrap();
    }
    println!("cnt: {}", *cnt.lock().unwrap());
}
