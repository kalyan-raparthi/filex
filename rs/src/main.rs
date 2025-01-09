mod http;

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[allow(dead_code)]

use http::parser::{parse_headers_to_hashmap, print_headers};

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

        stream.flush().unwrap();
        // NOTIFYING CLINECT REQUEST;
        stream.read(&mut buffer).expect(&format!("ERROR WHILE READING FROM {}", cli_addr));
        println!("REQUEST RECEIVED FROM: {}", cli_addr);
        let _request_headers = parse_headers_to_hashmap(String::from_utf8_lossy(&buffer).to_string());
        // print_headers(_request_headers);

        //CREATING BOILERPLATE FOR RESPONSE STRING
        let mut response_str = String::from(
            "<!DOCTYPE html>\
            <html>\
            <head><link rel='stylesheet' href='https://kalyan-raparthi.github.io/me/pages/style.css'></head>\
            <body>\
            <form action=\"/submit\" method=\"POST\">\
                <h1>ENTER: </h1><br>    
                <input name=\"message\" value=\"hello\">\
                <button type=\"submit\">Send</button>\
                </form>\
                <h1>Files</h1><br>"
            );

        // GET FILE_NAMES IN PWD
        let file_names = read_dir(".");
        for file_name in file_names {
            let temp_str = format!("<button>{}</button><br>", file_name);
            response_str.push_str(&temp_str);
        } 
        response_str.push_str("</body></html>");

        // Create HTTP response
        let response = format!(
            "HTTP/1.1 200 OK\n\
            Content-Type: text/html\n\
            Content-Length: {}\n\
            \n\
            {}",
            response_str.len(),
            response_str
        );
        
        // WRITING HTML RESPONSE AS TEXT/HTML
        stream.write_all(response.as_bytes()).expect("ERROR WHILE WRITING TO CLIENT");
        stream.flush().expect("ERROR WHILE FLUSHING");
        
        stream.read(&mut buffer).expect(&format!("ERROR WHILE READING FROM {}", cli_addr));
        let post_headers = parse_headers_to_hashmap(String::from_utf8_lossy(&buffer).to_string());
        stream.flush().unwrap();
        
        println!("{}", post_headers["message"]);
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

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);
    if request.starts_with("POST") {
        if let Some(body_start) = request.find("\r\n\r\n") {
            let body = &request[body_start + 4..];
            println!("Received POST body: {}", body);
        }
    }

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nResponse received!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}