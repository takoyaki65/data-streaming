use error::ServerError;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpListener};
use std::{thread, time};

pub mod error;

//* Protocol
// 1. create socket
// 2. connect to client
// 4. send stock_data to client
// 5. close connection

//* References
// https://zenn.dev/woden/articles/56a452bebb166a
// https://www.coins.tsukuba.ac.jp/~syspro/2024/2024-07-17/index.html
// Socket programmingでは、親サーバがaccept処理のみを行い、
// 子のサーバに処理を渡して、複数threadで処理を行なう

fn main() -> Result<(), ServerError> {
    // port
    let port = 5000;
    // 1.create socket
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port))?;
    println!("Server listening on port: {}", port);

    // connect
    loop {
        match listener.accept() {
            // connection success!
            Ok((mut socket, addr)) => {
                // split into multiple threads
                thread::spawn(move || {
                    println!("Client connected: {:?}", addr);
                    // send stock_data to client
                    for result in BufReader::new(File::open("stock_data.txt")?)
                        .lines()
                        .skip(1)
                    {
                        match result {
                            Ok(record) => match socket.write_all(record.as_bytes()) {
                                Ok(_) => {
                                    println!("Sent message to client: {}", record);
                                }
                                Err(e) => {
                                    eprintln!("Error writing to socket: {}", e);
                                    return Err(ServerError::IoError(e));
                                }
                            },
                            Err(e) => {
                                eprintln!("Error reading record: {}", e);
                                return Err(ServerError::IoError(e));
                            }
                        }
                        // sleep for 48 milliseconds
                        thread::sleep(time::Duration::from_millis(48));
                    }
                    // send Over
                    match socket.write_all("Over".as_bytes()) {
                        Ok(_) => {
                            println!("Sent message to client: Over");
                        }
                        Err(e) => {
                            eprintln!("Error writing to socket: {}", e);
                            return Err(ServerError::IoError(e));
                        }
                    }
                    // close connection
                    socket.shutdown(Shutdown::Both)?;
                    Ok(())
                });
            }
            // connection failed
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                return Err(ServerError::IoError(e));
            }
        }
    }
}
