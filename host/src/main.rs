use std::env;
use std::fs;
use std::io::Write;
use std::net::*;


fn main() {
    let args: Vec<String> = env::args().collect();  
    println!("hosting : {}", &args[1]);
    
    let data: Vec<u8> = fs::read(&args[1]).unwrap();
    
    // let addr: &str = "192.168.37.191:3034";
    let listener = TcpListener::bind(&args[2]).expect("bind");
    println!("listening on addr : {}", &args[2]);
    
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
