use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    const BUF_SIZE: usize = 4096; // BUFFER SIZE
    let mut buffer: [u8; BUF_SIZE] = [0; BUF_SIZE];

    println!("FILEX STARTED");
    let addr = "127.0.0.1:2025";

    let listener = TcpListener::bind(addr).expect("ERROR WHILE CREATING SOCKET");
    println!("SERVER IS LIVE AT: {}", addr);

    loop {
        // WAITING FOR REQUEST
        let (mut stream, cli_addr) = listener.accept().expect("ERROR OCCURRED WHILE LISTENING FOR CONNECTIONS");
        println!("CONNECTED TO CLIENT: {}", cli_addr);

        // NOTIFYING CLINECT REQUEST;
        stream.read(&mut buffer).expect(&format!("ERROR WHILE READING FROM {}", cli_addr));
        println!("REQUEST RECEIVED FROM: {}", cli_addr);

        // GET FILE_NAMES IN PWD
        let file_names = read_dir(".");

        //CREATING BOILERPLATE FOR RESPONSE STRING
        let mut response_str = String::from(
            "<!DOCTYPE html>\
            <html>\
            <head><link rel='stylesheet' href='https://kalyan-raparthi.github.io/me/pages/style.css'></head>\
            <body>\
            <h1>Files</h1><br>"
        );

        for file_name in file_names {
            let temp_str = format!("<button>{}</button><br>", file_name);
            response_str.push_str(&temp_str);
        } 

        response_str.push_str("</body></html>");

        // Create HTTP response
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
            Content-Type: text/html\r\n\
            Content-Length: {}\r\n\
            \r\n\
            {}",
            response_str.len(),
            response_str
        );

        // WRITING HTML RESPONSE AS TEXT/HTML
        stream.write_all(response.as_bytes()).expect("ERROR WHILE WRITING TO CLIENT");
    }
}

fn read_dir(path: &str) -> Vec<String> {
    let mut res = vec![];

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if let Some(file_name) = entry.file_name().to_str() {
                            res.push(file_name.to_string());
                        }
                    },
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        },
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
    
    res // RETURNING VEC CONTAINING FILE_NAMES
}
