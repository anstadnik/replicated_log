use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead, BufReader};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
// use std::time::Duration;

const SEC_IPS: [&'static str; 1] = ["http://sec1:8080"];

fn distribute_messages(line: String, clone: Arc<Mutex<Vec<String>>>) {
    let mut v = clone.lock().unwrap();
    // thread::sleep(Duration::from_secs(1));
    println!("You entered: {};", &line);

    let mut map = HashMap::new();
    map.insert("message", &line);

    let client = reqwest::blocking::Client::new();
    for ip in SEC_IPS {
        let _res = client.post(ip).json(&map).send();
        println!("Response is: {:?}", _res.ok().unwrap());
    }
    v.push(line);
}

fn handle_connection(
    stream: TcpStream,
    threads: &mut Vec<JoinHandle<()>>,
    v: &Arc<Mutex<Vec<String>>>,
) {
    let mut line = String::new();
    BufReader::new(stream).read_line(&mut line).unwrap();

    match line.trim() {
        "print" => println!("{:?}", v),
        l => threads.push(thread::spawn({
            let clone = Arc::clone(&v);
            let line = l.to_string();
            || distribute_messages(line, clone)
        })),
    };
}

fn tcp(threads: &mut Vec<JoinHandle<()>>, v: &Arc<Mutex<Vec<String>>>) {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, threads, v);
    }
}

fn console(threads: &mut Vec<JoinHandle<()>>, v: &Arc<Mutex<Vec<String>>>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line) = lines.next() {
        match line.unwrap().as_str() {
            "print" => println!("{:?}", v),
            l => threads.push(thread::spawn({
                let clone = Arc::clone(&v);
                let line = l.to_string();
                || distribute_messages(line, clone)
            })),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let v = Arc::new(Mutex::new(Vec::new()));
    let mut threads = Vec::new();

    if args.len() != 2 {
        println!("Please specify io type: {{\"console\", \"tcp\"}}");
        return;
    } else if args[1] == "console" {
        console(&mut threads, &v);
    } else if args[1] == "tcp" {
        tcp(&mut threads, &v);
    } else {
        return;
    }

    println!("{:?}", v);
    for t in threads {
        t.join().unwrap();
    }
    println!("{:?}", v);
}
