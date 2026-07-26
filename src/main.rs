use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..1001 {
            println!("Hi from the spawned thread (the number = {})", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..10 {
        println!("Hi from the main thread (the number = {})", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    println!("finish!");
}