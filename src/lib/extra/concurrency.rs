// 1:1 threads -> 1 os thread one program thread using os api

// green threads -> programming languagues own implementation
// also called M:N model,
// M green threads per N operating system threads
// where M and N are not necessarityl same

// every non-assembly will have some runtime

use std::{
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

pub fn spawner() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1))
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn move_example() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || println!("Here's a vector:  {:?}", v));

    handle.join().unwrap()
}

//
//
// WAYS OF HANDLING CONCURRENCY
//
//
// message passing to transfer data between threads with channel
//
// go - do not communicate by sharing memory, share memory by communicating

// mpsc - multiple producer, single consumer
// rust std implementation

pub fn channels() {
    // tx - transmitter / sender | rx - receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let receiverd = rx.recv().unwrap();
    println!("Got: {}", receiverd)
}

pub fn multiple_sends() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            "hi".to_string(),
            "from".to_string(),
            "the".to_string(),
            "thread".to_string(),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received)
    }
}

pub fn multiple_producer() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            "more".to_string(),
            "messages".to_string(),
            "for".to_string(),
            "you".to_string(),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            "hi".to_string(),
            "from".to_string(),
            "the".to_string(),
            "thread".to_string(),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received)
    }
}

//
//
// Shared-State Concurrency
//
// mutex/mutual exclusion
pub fn mutual_exclusion() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {}", m.lock().unwrap());
}

pub fn shared_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", counter.lock().unwrap());
}

// std::marker::Send + std::marker::Sync
