use std::{
    net::TcpStream,
    io::{Read, Write},
    str,
};

fn main() {
    let mut stream = TcpStream::connect("localhost:1145").unwrap();
    stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();

    println!(
        "response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
