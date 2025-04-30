use core::f64;
use error::ClientError;
use model::{StockData, StockEnum};
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};

pub mod error;
pub mod model;

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

    // submission start timer
    let mut start_time = chrono::Utc::now();
    println!("Start time: {}", start_time);
    // submission collect stock data
    let mut opens: Vec<f64> = vec![];
    let mut highs: Vec<f64> = vec![];
    let mut lows: Vec<f64> = vec![];
    let mut closes: Vec<f64> = vec![];

    // 3. receive message from server
    loop {
        let mut buffer = [0u8; 1024];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let record: Vec<String> = String::from_utf8_lossy(&buffer[..bytes_read])
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();
                // check if the message is "Over"
                if record[0] == "Over" {
                    break;
                }
                // create StockData object
                let stock = record[0].parse::<StockEnum>()?;
                let open = record[1].parse::<f64>()?;
                let high = record[2].parse::<f64>()?;
                let low = record[3].parse::<f64>()?;
                let close = record[4].parse::<f64>()?;
                let timestamp = chrono::NaiveDateTime::parse_from_str(
                    record[5].as_str(),
                    "%Y-%m-%d %H:%M:%S.%f",
                )?;
                let stock_data = StockData {
                    stock,
                    open,
                    high,
                    low,
                    close,
                    timestamp,
                };
                // collect stock data
                opens.push(stock_data.open);
                highs.push(stock_data.high);
                lows.push(stock_data.low);
                closes.push(stock_data.close);
                // get current time
                let current_time = chrono::Utc::now();
                if current_time > start_time + chrono::Duration::milliseconds(5100) {
                    // print result
                    let max_opens =
                        match opens
                            .iter()
                            .cloned()
                            .reduce(|a, b| match a.partial_cmp(&b) {
                                Some(std::cmp::Ordering::Greater) => a,
                                Some(std::cmp::Ordering::Equal) => b,
                                Some(std::cmp::Ordering::Less) => b,
                                None => a,
                            }) {
                            Some(v) => v,
                            None => return Err(ClientError::MaxValueNotFoundError),
                        };
                    let min_opens =
                        match opens
                            .iter()
                            .cloned()
                            .reduce(|a, b| match a.partial_cmp(&b) {
                                Some(std::cmp::Ordering::Greater) => b,
                                Some(std::cmp::Ordering::Equal) => a,
                                Some(std::cmp::Ordering::Less) => a,
                                None => b,
                            }) {
                            Some(v) => v,
                            None => return Err(ClientError::MinValueNotFoundError),
                        };
                    let max_highs =
                        match highs
                            .iter()
                            .cloned()
                            .reduce(|a, b| match a.partial_cmp(&b) {
                                Some(std::cmp::Ordering::Greater) => a,
                                Some(std::cmp::Ordering::Equal) => b,
                                Some(std::cmp::Ordering::Less) => b,
                                None => a,
                            }) {
                            Some(v) => v,
                            None => return Err(ClientError::MaxValueNotFoundError),
                        };
                    let min_highs =
                        match highs
                            .iter()
                            .cloned()
                            .reduce(|a, b| match a.partial_cmp(&b) {
                                Some(std::cmp::Ordering::Greater) => b,
                                Some(std::cmp::Ordering::Equal) => a,
                                Some(std::cmp::Ordering::Less) => a,
                                None => b,
                            }) {
                            Some(v) => v,
                            None => return Err(ClientError::MinValueNotFoundError),
                        };
                    let max_lows =
                        match lows.iter().cloned().reduce(|a, b| match a.partial_cmp(&b) {
                            Some(std::cmp::Ordering::Greater) => a,
                            Some(std::cmp::Ordering::Equal) => b,
                            Some(std::cmp::Ordering::Less) => b,
                            None => a,
                        }) {
                            Some(v) => v,
                            None => return Err(ClientError::MaxValueNotFoundError),
                        };
                    let min_lows =
                        match lows.iter().cloned().reduce(|a, b| match a.partial_cmp(&b) {
                            Some(std::cmp::Ordering::Greater) => b,
                            Some(std::cmp::Ordering::Equal) => a,
                            Some(std::cmp::Ordering::Less) => a,
                            None => b,
                        }) {
                            Some(v) => v,
                            None => return Err(ClientError::MinValueNotFoundError),
                        };
                    let max_closes =
                        match closes
                            .iter()
                            .cloned()
                            .reduce(|a, b| match a.partial_cmp(&b) {
                                Some(std::cmp::Ordering::Greater) => a,
                                Some(std::cmp::Ordering::Equal) => b,
                                Some(std::cmp::Ordering::Less) => b,
                                None => a,
                            }) {
                            Some(v) => v,
                            None => return Err(ClientError::MaxValueNotFoundError),
                        };
                    let min_closes =
                        match closes
                            .iter()
                            .cloned()
                            .reduce(|a, b| match a.partial_cmp(&b) {
                                Some(std::cmp::Ordering::Greater) => b,
                                Some(std::cmp::Ordering::Equal) => a,
                                Some(std::cmp::Ordering::Less) => a,
                                None => b,
                            }) {
                            Some(v) => v,
                            None => return Err(ClientError::MinValueNotFoundError),
                        };
                    println!("=======================================");
                    println!("system time (UTC): {}", chrono::Utc::now());
                    println!("open: MAX {}, MIN {}", max_opens, min_opens);
                    println!("high: MAX {}, MIN {}", max_highs, min_highs);
                    println!("low: MAX {}, MIN {}", max_lows, min_lows);
                    println!("close: MAX {}, MIN {}", max_closes, min_closes);
                    println!("=======================================");
                    // reset
                    start_time = current_time;
                    opens.clear();
                    highs.clear();
                    lows.clear();
                    closes.clear();
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
