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
        
        let mut file = File::open("index.html").expect("ERROR WHILE OPENING FILE");
        file.write_all(read_dir(".").as_bytes()).expect_err("ERROR WHILE WRITING TO FILE");

        stream.read(&mut buffer).expect(&format!("ERROR WHILE READING FROM {}", cli_addr).to_string());
        println!("REQUEST RECEIVED FROM: {}", cli_addr);

        let file_path = "index.html"; 
        let file_content = fs::read(file_path).expect("ERROR WHILE OPENING HTML FILE");

        let content_size = file_content.len();  

        let response = format!(
            "HTTP/1.1 200 OK\r\n\
            Content-Type: text/html\r\n\
            Content-Length: {}\r\n\
            \r\n", content_size + 512
        );

        stream.write_all(response.as_bytes()).expect("ERROR WHILE WRITING TO CLIENT");
        stream.write_all(&file_content).expect("ERROR WHILE SENDING HTML FILE CONTENT");        
    }

}

fn read_dir(path: &str) -> String{
    let mut res = String::new();
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let file_name = entry.file_name().into_string().expect("");
                        res.push_str(&file_name);
                    },
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        },
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
    return res.to_string();
}