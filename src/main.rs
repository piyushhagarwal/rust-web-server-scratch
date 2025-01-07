use std::{
    io::{ BufRead, BufReader, Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
};

// Server address and port configuration
const SERVER_ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1); // Localhost IP address
const SERVER_PORT: u16 = 8080; // Port number
const SOCKET_ADDR: SocketAddrV4 = SocketAddrV4::new(SERVER_ADDR, SERVER_PORT);

fn main() {
    println!("Hello from Server!");
    let listener = TcpListener::bind(SOCKET_ADDR).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(err) => {
                println!("Connection failed: {:?}", err);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&stream);
    let mut headers = Vec::new();
    let mut content_length = None;
    let mut body = String::new();
    
    // Read headers
    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).unwrap();
        
        // Check for end of headers
        if line.trim().is_empty() {
            break;
        }
        
        // Parse Content-Length header
        if line.to_lowercase().starts_with("content-length:") {
            content_length = line
                .split(':')
                .nth(1)
                .and_then(|len| len.trim().parse::<usize>().ok());
        }
        
        headers.push(line.trim().to_string());
    }
    
    println!("Headers: {:#?}", headers);
    
    // If the content length is present, read the body. The body is the remaining data in the stream. 
    // We first make a vector of bytes of the length of the content length, then read the body into the vector. 
    // Finally, we convert the vector to a string.

    if let Some(length) = content_length {
        let mut body_vec = vec![0u8; length];
        buf_reader.read_exact(&mut body_vec).unwrap();
        
        match String::from_utf8(body_vec) {
            Ok(body_str) => {
                body = body_str;
            }
            Err(_) => println!("Body contains non-UTF8 data"),
        }
    }

    println!("Body: {:#?}", body);
    
    // Send a basic response
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK";
    stream.write_all(response.as_bytes()).unwrap();
}