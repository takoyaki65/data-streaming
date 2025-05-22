use std::collections::VecDeque;
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};

use axum::extract::ws::WebSocket;

use crate::error::window::WindowError;
use crate::model::args::{self, ArgsSet};
use crate::model::stock_data::create_stock_data;
use crate::model::window_data::WindowData;
use crate::window::{count_window, time_window};

// protocol
// 1. create socket
// 2. connect to server
// 4. receive message from server until recieved "Over"

// Domain
const PORT: u16 = 5000;

pub async fn socket(args_set: ArgsSet, socket: &mut WebSocket) -> Result<(), WindowError> {
    println!("Args: {:?}", args_set);
    // 1.create socket
    let server_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), PORT);
    println!(
        "Server listening on: sw://{}:{}",
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        PORT
    );

    // 2. connect to server
    let mut stream = match TcpStream::connect(server_address) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Error connecting to server: {}", e);
            return Err(WindowError::IoError(e));
        }
    };

    // initialize id
    let mut id = 0;
    // set is_fisrt_flag
    let mut is_first_flag = true;
    // collect record buffer
    let mut stock_data_buffer: VecDeque<WindowData> = VecDeque::new();

    // 3. receive message from server
    loop {
        let mut buffer = [0u8; 1024];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                // get buffer
                let record: Vec<String> = String::from_utf8_lossy(&buffer[..bytes_read])
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();
                // check if the message is "Over"
                if record[0] == "Over" {
                    break;
                }
                // generate WindowData
                let window_data = WindowData::new(create_stock_data(record)?, id);
                id += 1;
                // ** sliding window process ** //
                match args_set.types {
                    // time based window
                    args::SlidingWindowEnumType::Count => {
                        count_window::count_window(
                            &mut is_first_flag,
                            &mut stock_data_buffer,
                            &args_set,
                            window_data,
                            socket,
                        )
                        .await?
                    }
                    // count based window
                    args::SlidingWindowEnumType::Time => {
                        time_window::time_window(
                            &mut is_first_flag,
                            &mut stock_data_buffer,
                            &args_set,
                            window_data,
                            socket,
                        )
                        .await?
                    }
                };
            }
            Err(e) => {
                eprintln!("Error receiving message from server: {}", e);
                return Err(WindowError::IoError(e));
            }
        }
    }
    Ok(())
}
