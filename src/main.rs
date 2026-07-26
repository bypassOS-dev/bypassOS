use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(String::from("Rust")).unwrap();
    });
    let message = rx.recv().unwrap();

    println!("{message}");
}
