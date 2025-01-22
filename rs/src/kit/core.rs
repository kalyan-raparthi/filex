use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::thread;

// CURRENT DIRECTORY PATH USED AS NAVIGATION ROOT
const HOME: &str = "C:/Users/Jahnavi";

// Starts the HTTPX server and listens for incoming connections.
pub fn app_start(ip: &str, port: &str) -> std::io::Result<()> {
    let address = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(address)?;
    println!("LISTENING: {}", listener.local_addr().expect("ERROR WHILE GETTING LOCAL ADDRESS"));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    response(stream);
                });
            }
            Err(e) => {
                eprintln!("CONNECTION_FAILED: {}", e);
            }
        }
    }
    Ok(())
}

/// Sends a response with the given status code, status message, and optional body.
pub fn send_response(writer: &mut BufWriter<&TcpStream>, status_code: u16, status_message: &str, body: String) {
    // let body = body;
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        status_message,
        body.len(),
        body
    );
    writer.write_all(response.as_bytes()).expect("send_response: ERROR WHILE WRITING RESPONSE");
    writer.flush().expect("send_response: ERROR WHILE FLUSHING BUFFER");
}

/// Generates the HTTP header for the given response type and file path.
pub fn get_header(response_type: &str, path: &str) -> String {
    let content_type = match response_type {
        "html" => "text/html",
        "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "png" => "image/png",
        "jpg" => "imag/jpeg",
        "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "ico" => "image/x-icon",
        "svg" => "image/svg+xml",
        "tiff" => "image/tiff",
        "webp" => "image/webp",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "mp4" => "video/mp4",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "rar" => "application/vnd.rar",
        "7z" => "application/x-7z-compressed",
        "txt" => "text/plain",
        "md" => "text/markdown",
        _ => "application/octet-stream",
    };
    
    let content_length = get_file_size(path).expect("get_header: ERROR WHILE GETTING FILE SIZE");
    
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        content_type, content_length
    )
}



/// Sends the requested file to the client.
pub fn send_file(writer: &mut BufWriter<&TcpStream>, path: &str) {
    println!("REQ_FILE: {}", path);
    let file = File::open(path).expect("ERROR WHILE OPENING FILE");
    
    let paths = path.split('/').last().unwrap();
    let response_type = paths.split('.').last().unwrap();
    // attaching specific headers to the response omg qb!
    writer.write_all(get_header(response_type, path).as_bytes()).expect("send_file: ERROR WHILE WRITING HEADERS TO CLIENT");
    std::io::copy(&mut BufReader::new(file), writer).expect("send_file: ERROR WHILE WRITING CONTENT TO CLIENT");
    writer.flush().expect("send_file: ERROR WHILE FLUSHING BUFFER");
}

fn get_file_size(path: &str) -> std::io::Result<u64> {
    let metadata = std::fs::metadata(path).expect("get_file_size: ERROR WHILE GETTING FILE METADATA");
    Ok(metadata.len())
}
pub fn response(stream: TcpStream) {
    print!("REQUEST: {} >>> ", stream.peer_addr().unwrap());

    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    
    let mut request = String::new();
    let _ = reader.read_line(&mut request);

    match request.split_whitespace().next().unwrap() {
        "GET" => { handle_get( &mut writer, request.split_whitespace().nth(1).unwrap());}
        _ => { eprintln!("INVALID HTTP METHOD"); }
    }
}

// Handles a GET request by sending the requested file or a 404 response if the file does not exist.
// ====================================== write handler for get request ======================================= //    
fn handle_get(writer: &mut BufWriter<&TcpStream>, path: &str) {
    println!("---------------------------------------------------------\nGET: {}", path);
    send_ftp_response(writer, path);
}

// HANDLE FTP RESPONSE
fn send_ftp_response(writer: &mut BufWriter<&TcpStream>, path: &str) {
    let path_for_dir = if path == "/" { 
        HOME
    } else { 
        &format!("{}{}", HOME, path)
    };  
    let metadata = fs::metadata(path_for_dir).expect("ERROR WHILE GETTING METADATA");  
    if metadata.is_dir() {
        println!("SENDING DIRECTORY: {}", path_for_dir);
        send_dir(writer, path_for_dir);
    } else if metadata.is_file() {
        println!("SENDING FILE: {}", path_for_dir);
        send_file(writer, path_for_dir);
    } else {
        send_response(writer, 404, "NOT FOUND", "404 NOT FOUND".to_string());
    }
}

// SEND DIR FUNCTION

// fn send_dir(writer: &mut BufWriter<&TcpStream>, path: &str) {
//     let mut body = String::new();
    
//     let dir = match std::fs::read_dir(path) {
//         Ok(dir) => dir,
//         Err(_) => {
//             send_response(writer, 500, "INTERNAL SERVER ERROR", "500 INTERNAL SERVER ERROR".to_string());
//             return;
//         }
//     };
    
//     for entry in dir.filter_map(Result::ok) {
//         let name = match entry.file_name().into_string() {
//             Ok(name) => name,
//             Err(_) => continue,
//         };
//         body.push_str(&format!("<a href=\"{}{}\">{}</a><br>", path, name, name));
//     }
//     send_response(writer, 200, "OK", body);
// }

fn send_dir(writer: &mut BufWriter<&TcpStream>, path: &str) {
    let mut body = String::new();

    let dir = match std::fs::read_dir(path) {
        Ok(dir) => dir,
        Err(_) => {
            send_response(writer, 500, "INTERNAL SERVER ERROR", "500 INTERNAL SERVER ERROR".to_string());
            return;
        }
    };

    for entry in dir.filter_map(Result::ok) {
        let name = match entry.file_name().into_string() {
            Ok(name) => name,
            Err(_) => continue,
        };

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };
        // REMOVING HOME DIRECTORY FROM PATH SO THAT IT CAN BE USED AS RELATIVE PATH
        let relative_path = path.strip_prefix(HOME).unwrap_or(path);
        let relative_path = format!("{}/{}", relative_path, name);

        if metadata.is_dir() {
            body.push_str(&format!("<a href=\"{}\">{}/</a><br>", relative_path, name));
        } else {
            body.push_str(&format!("<a href=\"{}\">{}</a> ({})<br>", relative_path, name, metadata.len()));
        }
    }
    send_response(writer, 200, "OK", body);
}