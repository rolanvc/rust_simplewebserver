use std::io::prelude::*;
use std::net::{TcpListener,TcpStream };
use std::io::BufReader;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
            handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
        // Read all the headers
        // for header in BufReader::new(&mut stream).lines() {
        //    let header = header.unwrap();
        //    if header == "\r" { break }
        // }
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();
    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}