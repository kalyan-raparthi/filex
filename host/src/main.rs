use std::env;
use std::fs;
use std::io::Write;
use std::net::*;

fn main() {
    const ADDR: &str = "127.0.0.1:20240";
    let args: Vec<String> = env::args().collect();
    println!("hosting : {}", args[1]);

    let data: Vec<u8> = fs::read(&args[1]).unwrap();
    
    let listener = TcpListener::bind(ADDR).expect("bind");
    println!("listening on addr : {ADDR}");
    
    for stream in listener.incoming()  {
        match stream {
            Ok(mut stream) => {
                println!("file request");
                stream.write(&data).expect("error while writing");

            }
            Err(e) => {
                println!("{e:?}");
            }
        }
    }

}

