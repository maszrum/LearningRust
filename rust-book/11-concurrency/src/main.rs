use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    spawning_thread();
    mpsc_channel();
    mutex();
}

fn spawning_thread() {
    let vec = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in &vec {
            println!("hi number {} from spawned thread", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn mpsc_channel() {
    let (tx, rx) = mpsc::channel();

    let second_tx = tx.clone();

    thread::spawn(move || {
        let messages = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("channel")
        ];

        for message in messages {
            tx.send(Some(message)).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        tx.send(None).unwrap();
    });

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));

        let message = String::from("message from second tx");
        second_tx.send(Some(message)).unwrap();
    });

    for message in rx {
        match message {
            Some(text) => println!("got: {}", text),
            None => break
        }
    }
}

fn mutex() {
    let m = Arc::new(Mutex::new(5));
    let mut handles = vec![];

    for _i in 0..10 {
        let counter = Arc::clone(&m);

        let handle = thread::spawn(move || {
            let mut m = counter.lock().unwrap();
            *m += 1;
        });

        handles.push(handle);
    }

    println!("m = {:?}", m);
}
