//use std::rc::Rc;
use std::sync::mpsc;
use std::sync::Arc; // Atomic Rc (Reference Counted pointer)
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Chapter 16: Fearless Concurrency");
    section16_1();
    section16_2();
    section16_3();
    section16_4();
}

fn section16_1() {
    println!("Section 16.1 Run Code Simultaneously");

    let v = vec![1, 2, 3];

    let thread_handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }

        println!("{v:?}");
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    thread_handle.join().unwrap();
    //println!("{v:?}");
}

fn section16_2() {
    println!("Section 16.2 Message Passing");

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let messages = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for m in messages {
            tx.send(m).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("{received}");
    }

    // can also use tx.clone() to make multiple producers to rx, not going to bother implementing
}

fn section16_3() {
    println!("Section 16.3 Shared State Concurrency");

    let m = Mutex::new(6);

    {
        let mut num = m.lock().unwrap();
        *num = 5;
    }

    println!("m = {m:?}");

    {
        //let counter = Rc::new(Mutex::new(0));
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            //let counter = Rc::clone(&counter);
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

        println!("Result: {}", *counter.lock().unwrap());
    }

}

fn section16_4() {
    println!("Section 16.4 Extensible Concurrency");
    println!("Just reading ...");
}