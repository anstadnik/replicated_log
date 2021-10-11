use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let v = Arc::new(Mutex::new(Vec::new()));
    let mut threads = Vec::new();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line) = lines.next() {
        threads.push(thread::spawn({
            let clone = Arc::clone(&v);
            move || {
                let line = line.unwrap();
                let mut v = clone.lock().unwrap();
                thread::sleep(Duration::from_secs(1));
                println!("You entered: {}", line);
                v.push(line);
            }
        }));
    }

    println!("{:?}", v);
    for t in threads {
        t.join().unwrap();
    }
    println!("{:?}", v);
}
