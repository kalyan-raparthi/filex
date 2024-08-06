// use std::env;
// use std::fs;
use std::net::*;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let data: Vec<u8>     = fs::read(&args[1]).unwrap();
    // println!("contents : {:?}", data);
    
    let listener = TcpListener::bind("127.0.0.1:2024").unwrap();
    
    match listener.accept() {
        Ok((_socket, addr)) => println!("new client {addr:?}"),
        Err(e) => println!("{e:?}")
    }
}

