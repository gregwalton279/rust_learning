use std::env::join_paths;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mut x = 100;
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move|| {
        // x 被 move进 spawn中后已与外面的x不同了。
        x = 9;
        v;
        for i in 1..=x {
            println!("spawn thread:: ---> {}", i);
            thread::sleep(Duration::from_millis(1));
        }

        return "hello thread";
    });

    println!("x is moved {}", x);
    // println!("v is moved {:?}", v); // error

    for i in 1..5 {
        println!("main thred:: ---> {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // thread::yield_now();
    // thread::park();

    println!("handle thread {}", handle.join().unwrap());

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = String::from("hi");
        tx1.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("received from main thread: {}", received);

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let co = counter.clone();
        let handle = thread::spawn(move|| {
            let mut num = co.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
