use std::fs::*;
use std::io::Read;
use std::io::Write;
use std::net::*;
use std::env;

fn main() {
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut result: [char; 1024] = [' '; 1024];

    let args: Vec<String> = env::args().collect();
    let mut stream = TcpStream::connect(&args[1]).expect("could not connect to host");
    

    let mut file = OpenOptions::new().write(true).open("result.txt").expect("file opening");

    stream.read(&mut buffer).expect("error while reading");
    for (i, &byte) in buffer.iter().enumerate() {
        if byte == 0 {
            break;
        }
        result[i] = byte as char;
    }

    file.write(&buffer).expect("while writing to file");
}