use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpListener};

#[derive(Debug, thiserror::Error)]
enum ServerError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

// protocol
// 1. create socket
// 2. connect to client
// 3. get from client messge
// 4. send message to client
// 5. close connection

fn main() -> Result<(), ServerError> {
    // port
    let port = 5000;
    // 1.create socket
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port))?;
    println!("Server listening on port: {}", port);

    // connect
    match listener.accept() {
        // connection success!
        Ok((mut socket, addr)) => {
            println!("Client connected: {:?}", addr);

            // read from client message!
            // create buffer
            let mut buffer = [0; 1024];
            match socket.read(&mut buffer) {
                Ok(bytes_read) => {
                    let recieved_message = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("Recieved message: {}", recieved_message);
                }
                Err(e) => {
                    eprintln!("Error reading from socket: {}", e);
                    return Err(ServerError::IoError(e));
                }
            }

            // send message to client
            let response_message = "Hello from server!";
            match socket.write_all(response_message.as_bytes()) {
                Ok(_) => {
                    println!("Sent message to client: {}", response_message);
                }
                Err(e) => {
                    eprintln!("Error writing to socket: {}", e);
                    return Err(ServerError::IoError(e));
                }
            }

            // close connection
            socket.shutdown(Shutdown::Both)?;
        }
        // connection failed
        Err(e) => {
            eprintln!("Error accepting connection: {}", e);
            return Err(ServerError::IoError(e));
        }
    }
    Ok(())
}
