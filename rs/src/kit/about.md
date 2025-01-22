This is a http utilities package. 

Author: kalyan rapathi / [ qb ], kalyan.raparthi@hotmail.com, Github: kalyan-raparthi

`  core  `:
```
    - Starts the HTTPX server and listens for incoming connections.
        pub fn app_start() -> std::io::Result<()>;

    - Checks if a file exists at the given path.
        pub fn file_exists(path: &str) -> bool;

    - Sends a response with the given status code, status message, and optional body.
        pub fn send_response(writer: &mut BufWriter<&TcpStream>, status_code: u16, status_message: &str, body: Option<&str>);

    - Generates the HTTP header for the given response type and file path.
        pub fn get_header(response_type: &str, path: &str) -> String;

    - Handles a GET request by sending the requested file or a 404 response if the file does not exist.
        pub fn handle_get(writer: &mut BufWriter<&TcpStream>, path: &str);

    - Sends the requested file to the client.
        pub fn send_file(writer: &mut BufWriter<&TcpStream>, path: &str);

    - Gets the size of the file at the given path.
        fn get_file_size(path: &str) -> std::io::Result<u64>;

    - Handles the response for an incoming TCP stream.
        pub fn response(stream: TcpStream);
```