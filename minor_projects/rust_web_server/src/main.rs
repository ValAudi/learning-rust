use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
   let listener =  TcpListener::bind("127.0.0.1:7878").unwrap();
   // println!("{:?}", listener)
   for stream_counter in listener.incoming() {
       let stream = stream_counter.unwrap();
       // println!("{:?}", stream);
       // println!("Connection established!")
       manage_connecion(stream);
   }
}

fn manage_connecion (mut stream: TcpStream) {
    let mut buffer = [0;1024];  // initialize a buffer on the stack
    stream.read(&mut buffer).unwrap(); // read the contents of the stream onto the buffer
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..])); // read from the buffer
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    }; // good exapmple of many variable bindings
    
    let contents = fs::read_to_string(filename).unwrap();
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // let answer = format!("HTTP/1.1 200 OK\r\nContent-Lenght: {}\r\n\r\n{}", contents.len(), contents);
    let answer = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}", 
        status_line,
        contents.len(),
        contents
    );
    stream.write(answer.as_bytes()).unwrap();
    stream.flush().unwrap();
}
