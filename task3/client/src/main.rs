use std::collections::VecDeque;
use std::env;
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use utils::error::ClientError;
use utils::model::WindowData;
use utils::stock_data::create_stock_data;
use utils::{count_window, stat, time_window};

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
    // 1-1. treats as u64
    // [terms about count]
    // 1. window and slide are both (numbers with 2 decimal places) ∧ (n ∈ R).
    // 1-1. treats as f64
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
                // generate StockData
                let stock_data = create_stock_data(record)?;
                // generate WindowData
                let window_data = WindowData::new(stock_data.clone());
                // show get record
                println!("{:?}", stock_data);
                // ** sliding window process ** //
                match args_set.types {
                    utils::args::SlidingWindowEnumType::Count => {
                        if is_first_flag {
                            // first window process
                            if stock_data_buffer.is_empty() {
                                // start window
                                println!("------------------------------------------");
                                println!("Start Window [{}]", chrono::Utc::now());
                                println!("------------------------------------------");
                            }
                            if stock_data_buffer.len() < args_set.get_window_count_value()? as usize
                            {
                                // push back WindowData
                                stock_data_buffer.push_back(window_data);
                            } else {
                                is_first_flag = false;
                            }
                            if !is_first_flag {
                                // show result
                                stat::show_stat(stock_data_buffer.clone())?;
                                // pop over record
                                for _ in 0..args_set.get_slide_count_value()? {
                                    // pop first element
                                    stock_data_buffer.pop_front();
                                }
                            }
                        } else {
                            // other window process
                            if stock_data_buffer.len() < args_set.get_window_count_value()? as usize
                            {
                                // push back WindowData
                                stock_data_buffer.push_back(window_data);
                            } else {
                                // show result
                                stat::show_stat(stock_data_buffer.clone())?;
                                // pop over record
                                for _ in 0..args_set.get_slide_count_value()? {
                                    // pop first element
                                    stock_data_buffer.pop_front();
                                }
                                // start window
                                println!("------------------------------------------");
                                println!("Start Window [{}]", chrono::Utc::now());
                                println!("------------------------------------------");
                            }
                        }
                    }
                    utils::args::SlidingWindowEnumType::Time => {
                        // check if the time is over
                        if is_first_flag {
                            // first window process
                            if stock_data_buffer.is_empty() {
                                // start window
                                println!("------------------------------------------");
                                println!("Start Window [{}]", chrono::Utc::now());
                                println!("------------------------------------------");
                                stock_data_buffer.push_back(WindowData::new(stock_data));
                            } else {
                                // get first element
                                let first_element = match stock_data_buffer.front() {
                                    Some(first_element) => first_element.clone(),
                                    None => {
                                        eprintln!("Error: first element is not found.");
                                        return Err(ClientError::PushFailedError);
                                    }
                                };
                                // get time difference
                                let time_diff =
                                    window_data.get_timestamp() - first_element.get_timestamp();
                                // check if the time is over
                                if time_diff.num_milliseconds()
                                    < args_set.get_window_time_value()? as i64
                                {
                                    // push back WindowData
                                    stock_data_buffer.push_back(window_data);
                                } else {
                                    is_first_flag = false;
                                }
                                if !is_first_flag {
                                    // show result
                                    stat::show_stat(stock_data_buffer.clone())?;
                                    // pop over record
                                    let pop_slide_time = first_element.get_timestamp()
                                        + chrono::Duration::milliseconds(
                                            args_set.get_slide_time_value()? as i64,
                                        );
                                    let mut index = 0;
                                    while pop_slide_time - stock_data_buffer[index].get_timestamp()
                                        > chrono::Duration::zero()
                                    {
                                        // pop front element
                                        stock_data_buffer.pop_front();
                                        index += 1;
                                        if index > stock_data_buffer.len() - 1 {
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            // other window process
                            // get first element
                            let first_element = match stock_data_buffer.front() {
                                Some(first_element) => first_element.clone(),
                                None => {
                                    eprintln!("Error: first element is not found.");
                                    return Err(ClientError::PushFailedError);
                                }
                            };
                            let last_element = match stock_data_buffer.back() {
                                Some(last_element) => last_element.clone(),
                                None => {
                                    eprintln!("Error: last element is not found.");
                                    return Err(ClientError::PushFailedError);
                                }
                            };
                            if last_element.get_timestamp() - first_element.get_timestamp()
                                < chrono::Duration::milliseconds(
                                    args_set.get_window_time_value()? as i64
                                )
                            {
                                // push back WindowData
                                stock_data_buffer.push_back(window_data);
                            } else {
                                // show result
                                stat::show_stat(stock_data_buffer.clone())?;
                                // pop over record
                                let pop_slide_time = first_element.get_timestamp()
                                    + chrono::Duration::milliseconds(
                                        args_set.get_slide_time_value()? as i64,
                                    );
                                let mut index = 0;
                                while pop_slide_time - stock_data_buffer[index].get_timestamp()
                                    > chrono::Duration::zero()
                                {
                                    // pop front element
                                    stock_data_buffer.pop_front();
                                    index += 1;
                                    if index > stock_data_buffer.len() - 1 {
                                        break;
                                    }
                                }
                                // start window
                                println!("------------------------------------------");
                                println!("Start Window [{}]", chrono::Utc::now());
                                println!("------------------------------------------");
                            }
                        }
                    }
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
