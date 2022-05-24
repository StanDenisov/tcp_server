use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::time::Duration;
use tcp_server::ThreadPool;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:38000").unwrap();
    let pool = ThreadPool::new(4);
    
    for stream in listner.incoming() {
        let stream = stream.unwrap();
        pool.execute( || {
        handle_connection(stream);
    });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let x = stream.set_read_timeout(Some(Duration::new(12, 12)));
    loop {
    let mut buffer = [0; 512];
    let x = stream.read(&mut buffer);
    match x {
        Ok(_) => println!("OK"),
        Err(_) => {
            println!("omg this is time out");
            break;
        }
    }
    println!("Запрос {}", String::from_utf8_lossy(&buffer[..]));
    }
}
