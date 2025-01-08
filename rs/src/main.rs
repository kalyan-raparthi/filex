use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::TcpListener;
// use std::path;

fn main() {
    const BUF_SIZE: usize = 4096; // BUFFER SIZE
    let mut buffer: [u8; BUF_SIZE] = [0; BUF_SIZE];

    println!("FILEX STARTED");
    let addr = "127.0.0.1:2025";

    let listener: TcpListener = TcpListener::bind(addr).expect("ERROR WHILE CREATING SOCKET");
    println!("SERVER IS LIVE AT: {}", addr);

    
    loop {
        let (mut stream, cli_addr) = listener.accept().expect("ERROR OCCURRED WHILE LISTING FROM CONNECTIONS");
        println!("CONNECTED TO CLIENT: {}", cli_addr);

        stream.read(&mut buffer).expect(&format!("ERROR WHILE READING FROM {}", cli_addr).to_string());
        println!("REQUEST RECEIVED FROM: {}", cli_addr);

        let file_names = read_dir(".");
        let mut response_str = String::from("
        <!DOCTYPE html> 
        <style> * { font-family: monospace; } </style>

        ");

        for i in file_names {
            response_str.push_str(&i);
            response_str.push('\n');
        } 

        let response = format!(
            "HTTP/1.1 200 OK\r\n\
            Content-Type: text/html\r\n\
            Content-Length: {}\r\n\
            \r\n", 512
        );

        stream.write_all(response.as_bytes()).expect("ERROR WHILE WRITING TO CLIENT");
        stream.write_all(response_str.as_bytes()).expect("ERROR WHILE SENDING HTML FILE CONTENT");        
    }

}

fn read_dir(path: &str) -> Vec<String>{
    let mut res = vec![];

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let file_name = entry.file_name().into_string().expect("FILE ERROR");
                        res.push(file_name);
                    },
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        },
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
    return res;
}