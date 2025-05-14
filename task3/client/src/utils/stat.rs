use super::window_data::WindowData;
use crate::utils::{error::ClientError, stock_data::StockEnum};
use statrs::statistics::Statistics;
use std::collections::{HashMap, VecDeque};

pub fn show_stat(stock_data_buffer: VecDeque<WindowData>) -> Result<(), ClientError> {
    println!("-Total Results----------------------");
    let mut hash_map: HashMap<StockEnum, Vec<f64>> = HashMap::new();
    for v in stock_data_buffer.iter() {
        let stock_kind: StockEnum = v.stock_data.stock.clone();
        let value: &mut Vec<f64> = hash_map.entry(stock_kind).or_default();
        value.push(v.stock_data.close);
    }
    let mut stocks: Vec<(String, Vec<f64>)> = hash_map
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect::<Vec<(String, Vec<f64>)>>();
    stocks.sort_by(|(key_a, _), (key_b, _)| key_a.cmp(key_b));
    stocks.into_iter().for_each(|(key, value)| {
        println!(
            "{} Max: {}, Min: {}, Mean: {}, Std: {}",
            key,
            value.clone().max(),
            value.clone().min(),
            value.clone().mean(),
            value.clone().std_dev()
        )
    });
    println!("--------------------------------------");
    // end window
    let end_window_time = stock_data_buffer
        .back()
        .ok_or(ClientError::PushFailedError)?
        .get_timestamp();
    println!("--------------------------------------");
    println!("End Window [{}]", end_window_time);
    println!("--------------------------------------");
    Ok(())
}
