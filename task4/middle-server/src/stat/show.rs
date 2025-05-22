use crate::{
    error::window::WindowError,
    model::{
        args::{ArgsSet, SlidingWindowEnumType},
        window_data::WindowData,
    },
};
use statrs::statistics::Statistics;
use std::collections::VecDeque;

pub fn show_stat_result(
    stock_data_buffer: VecDeque<WindowData>,
    args_set: ArgsSet,
    stocks: Vec<(String, Vec<f64>)>,
) -> Result<(), WindowError> {
    //* print result *//
    for stock_data in stock_data_buffer.clone() {
        println!("{:?}", stock_data);
    }
    println!("-Total Results----------------------");
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
    // show border window data info
    match args_set.types {
        SlidingWindowEnumType::Count => {
            let start_window_id = stock_data_buffer
                .front()
                .ok_or(WindowError::PushFailedError)?
                .get_id();
            let end_window_id = stock_data_buffer
                .back()
                .ok_or(WindowError::PushFailedError)?
                .get_id();
            println!("--------------------------------------");
            println!("End Window [window id: {}]", start_window_id);
            println!("End Window [window id: {}]", end_window_id);
            println!("--------------------------------------");
        }
        SlidingWindowEnumType::Time => {
            let start_window_time = stock_data_buffer
                .front()
                .ok_or(WindowError::PushFailedError)?
                .get_timestamp();
            let end_window_time = stock_data_buffer
                .back()
                .ok_or(WindowError::PushFailedError)?
                .get_timestamp();
            println!("--------------------------------------");
            println!("Start Window [timestamp: {}]", start_window_time);
            println!("End Window [timestamp: {}]", end_window_time);
            println!("--------------------------------------");
        }
    }
    Ok(())
}
