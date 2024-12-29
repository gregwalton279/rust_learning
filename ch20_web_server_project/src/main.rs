use std::{fs, thread};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use ch20_web_server_project::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        /// 为每一个请求分配线程的代码结构
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
        // handle_connection(stream);
        // println!("Connection established!");

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "ch20_web_server_project/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    }  else {
        ("HTTP/1.1 404 NOT FOUND", "ch20_web_server_project/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connectionOld(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    if request_line.starts_with("GET / HTTP/1.1") {
        let crlf = "\r\n\r\n";
        let status_line = "HTTP/1.1 200 OK";
        let content = fs::read_to_string("ch20_web_server_project/hello.html").unwrap();
        let length = content.len();
        let response = format!("{status_line}\r\nContent-Length: {length}{crlf}{content}");
        stream.write(response.as_bytes()).unwrap();
    } else {
        let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{request_line}");
    }


    // stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    // println!("http_request is : {:?}", http_request);
}