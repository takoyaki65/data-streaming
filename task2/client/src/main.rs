use error::ClientError;
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};

pub mod error;

// protocol
// 1. create socket
// 2. connect to server
// 3. receive message from server until recieved "Over"

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

    // 3. receive message from server
    loop {
        let mut buffer = [0u8; 1024];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let record = String::from_utf8_lossy(&buffer[..bytes_read]);
                if record == "Over" {
                    println!(
                        "{}, System Time: {}",
                        record,
                        chrono::Utc::now().naive_utc()
                    );
                    break;
                } else {
                    println!(
                        "{}, System Time: {}",
                        record,
                        chrono::Utc::now().naive_utc()
                    );
                }
            }
            Err(e) => {
                eprintln!("Error receiving message from server: {}", e);
                return Err(ClientError::IoError(e));
            }
        }
    }
    Ok(())
}
