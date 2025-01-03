use std::{io::{Read, Write}, net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream}};

// Server address and port configuration
// Define the server's IP address as localhost (127.0.0.1) and the port number as 8080
const SERVER_ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1); // Localhost IP address
const SERVER_PORT: u16 = 8080; // Port number
const SOCKET_ADDR: SocketAddrV4 = SocketAddrV4::new(SERVER_ADDR, SERVER_PORT); // Combine the IP and port into a socket address

fn main() {
    // Print a message indicating that the server is starting
    println!("Hello from Server!");

    // Step 1: Bind the listener to the specified address and port
    // Create a TcpListener bound to the defined server address and port
    let listener = TcpListener::bind(SOCKET_ADDR).unwrap();

    // Step 2: Infinite loop to accept incoming connections
    // The server continuously listens for incoming TCP connections
    for stream in listener.incoming() {
        match stream {
            // If a connection is successfully accepted:
            Ok(stream) => {
                // Print the address of the incoming connection
                println!("New connection: {}", stream.peer_addr().unwrap());
                // Handle the incoming client by passing the stream to a handler function
                handle_client(stream);
            }

            // If an error occurs while accepting a connection:
            Err(err) => {
                // Print the error message
                println!("Connection failed: {:?}", err);
            }
        }
    }
}

// Function to handle the client connection
fn handle_client(mut stream: TcpStream) {
    // Step 1: Declare a buffer to store data read from the stream
    let mut buffer = [0; 512]; // Create a buffer of 512 bytes

    // Step 2: Read data from the stream into the buffer
    match stream.read(&mut buffer) {
        // If the read operation is successful:
        Ok(size) => {
            // Print the received data as a string, converting the byte slice to UTF-8
            // 'size' is the number of bytes actually read
            println!("Received: {}", String::from_utf8_lossy(&buffer[0..size]));
            
            // Uncomment the line below to echo the received data back to the client
            // stream.write(&buffer[..size]).unwrap(); // Echo back the received data
        }
        
        // If an error occurs while reading from the stream:
        Err(e) => {
            // Print an error message
            println!("Failed to read from socket: {}", e);
        }
    }
}
