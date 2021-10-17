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
    map.insert("mes", &line);

    let client = reqwest::blocking::Client::new();
    for ip in SEC_IPS {
        let _res = client.post(ip).json(&map).send();
        match _res.ok() {
            Some(l) => println!("Response is: {:?}", l),
            None => println!("No response!"),
        }
    }
    v.push(line);
}

fn handle_connection(
    stream: TcpStream,
    threads: &mut Vec<JoinHandle<()>>,
    v: &Arc<Mutex<Vec<String>>>,
) -> io::Result<()> {
    let mut line = String::new();
    let mut reader = BufReader::new(stream);
    reader.read_line(&mut line)?;
    let get = "GET / HTTP/1.1".to_string();
    let post = "POST / HTTP/1.1".to_string();

    if line.starts_with(&get) {
        println!("{:?}", v);
    } else if line.starts_with(&post) {
        reader.read_line(&mut line)?;
        threads.push(thread::spawn({
            let line = line.trim().to_string();
            let clone = Arc::clone(&v);
            || distribute_messages(line, clone)
        }));
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Unknown request",
        ));
    }
    Ok(())
}

fn tcp(threads: &mut Vec<JoinHandle<()>>, v: &Arc<Mutex<Vec<String>>>) {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        match handle_connection(stream, threads, v) {
            Err(_) => println!("Unknown connection!"),
            _ => (),
        }
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
