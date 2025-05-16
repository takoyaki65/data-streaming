use std::collections::VecDeque;
use std::env;
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use utils::error::ClientError;
use utils::stock_data::create_stock_data;
use utils::window_data::WindowData;
use utils::{count_window, time_window};

pub mod utils;

// protocol
// 1. create socket
// 2. connect to server
// 4. receive message from server until recieved "Over"

// port
const PORT: u16 = 5000;

fn main() -> Result<(), ClientError> {
    // 0. get command line args
    // format: cargo run -- --(count|window) -- --window 5 -- --slide 2
    // [terms about count]
    // 1. window and slide are both (n > 0) ∧ (n ∈ N).
    // [terms about time]
    // 1. window and slide are both (accuracy: milliseconds) ∧ (n ∈ R).
    // 2. window > slide
    let args_set = utils::args::parse_args(env::args().collect())?;
    println!("Args: {:?}", args_set);
    // 1.create socket
    let server_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), PORT);
    println!("Server listening on: sw://localhost:{}", PORT);

    // 2. connect to server
    let mut stream = match TcpStream::connect(server_address) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Error connecting to server: {}", e);
            return Err(ClientError::IoError(e));
        }
    };

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
                let window_data = WindowData::new(create_stock_data(record)?);
                // ** sliding window process ** //
                match args_set.types {
                    utils::args::SlidingWindowEnumType::Count => count_window::count_window(
                        &mut is_first_flag,
                        &mut stock_data_buffer,
                        &args_set,
                        window_data,
                    )?,
                    utils::args::SlidingWindowEnumType::Time => time_window::time_window(
                        &mut is_first_flag,
                        &mut stock_data_buffer,
                        &args_set,
                        window_data,
                    )?,
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
