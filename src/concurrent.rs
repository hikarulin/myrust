use std::thread;
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub fn multi_thread() {
    let handler1 = thread::spawn(move ||{
        thread::sleep(Duration::from_secs(1));
        let val = String::from("hi");
    });
    println!("{:?}: waiting thread to be finished",Instant::now());
    handler1.join().unwrap();
    println!("{:?}: spawn thread finished",Instant::now());
}

pub fn multi_thread2() {
    let (sender,receiver) = mpsc::channel();
    thread::spawn(move ||{
        thread::sleep(Duration::from_secs(1));
        let val = String::from("hi");
        println!("{:?}: spawn thread send...", Instant::now());
        sender.send(val).unwrap();
        println!("{:?}: spawn thread send success", Instant::now());
    });
    println!("{:?}: main thread start receiving...",Instant::now());
    let val = receiver.recv().unwrap();
    println!("{:?}: main thread received:{}", Instant::now(), val);
}

pub fn multi_thread3() {
    let (sender,receiver) = mpsc::channel();
    thread::spawn(move ||{
        thread::sleep(Duration::from_secs(1));
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            println!("{:?}: spawn thread send {}...", Instant::now(),val);
            sender.send(val).unwrap();
        }
        println!("{:?}: spawn thread send success", Instant::now());
    });
    println!("{:?}: main thread sleep 2s...",Instant::now());
    thread::sleep(Duration::from_secs(2));
    for val in receiver {
        println!("{:?}: main thread received:{}", Instant::now(), val);
    }
}

pub fn multi_thread4() {
    let (sender,receiver) = mpsc::channel();
    let sender2 = sender.clone();
    thread::spawn(move ||{
        thread::sleep(Duration::from_secs(1));
        let vals = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi"),
            String::from("jkl"),
        ];
        for val in vals {
            println!("{:?}: spawn thread1 send {}...", Instant::now(),val);
            sender.send(val).unwrap();
        }
        println!("{:?}: spawn thread1 send success", Instant::now());
    });
    thread::spawn(move ||{
        thread::sleep(Duration::from_secs(1));
        let vals = vec![
            String::from("123"),
            String::from("456"),
            String::from("789"),
            String::from("000"),
        ];
        for val in vals {
            println!("{:?}: spawn thread2 send {}...", Instant::now(),val);
            sender2.send(val).unwrap();
        }
        println!("{:?}: spawn thread2 send success", Instant::now());
    });
    println!("{:?}: main thread sleep 2s...",Instant::now());
    thread::sleep(Duration::from_secs(2));
    for val in receiver {
        println!("{:?}: main thread received:{}", Instant::now(), val);
    }
}