use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

// protocol
// 1. create socket
// 2. connect to server
// 3. send message to server
// 4. receive message from server
// 5. close connection

fn main() -> Result<(), ClientError> {
    // port
    let port = 5000;
    // 1.create socket
    let server_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port);
    println!("Server listening on port: {}", port);

    // 2. connect to server
    let mut stream = match TcpStream::connect(server_address) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Error connecting to server: {}", e);
            return Err(ClientError::IoError(e));
        }
    };

    // 3. send message to server
    let message = "Hello from client!";
    match stream.write_all(message.as_bytes()) {
        Ok(_) => println!("Sent message to server: {}", message),
        Err(e) => {
            eprintln!("Error sending message to server: {}", e);
            return Err(ClientError::IoError(e));
        }
    }

    // 4. receive message from server

    let mut buffer = [0u8; 1024];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let recieved_message = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Received message from server: {}", recieved_message);
        }
        Err(e) => {
            eprintln!("Error receiving message from server: {}", e);
            return Err(ClientError::IoError(e));
        }
    }
    Ok(())
}
