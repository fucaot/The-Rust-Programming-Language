use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::{self, Thread},
};
use web::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.excute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = "GET / HTTP/1.1\r\n";

    if buffer.starts_with(get.as_bytes()) {
        println!("202!--");
        let contents = fs::read_to_string("hello.html").unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        // 等待并阻止程序的运行，直到所有数据被写入
        stream.flush().unwrap();
    } else {
        println!("404!--");
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        // 等待并阻止程序的运行，直到所有数据被写入
        stream.flush().unwrap();
    }
}
