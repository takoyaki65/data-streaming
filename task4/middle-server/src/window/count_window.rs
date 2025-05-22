use axum::extract::ws::WebSocket;

use crate::{
    error::window::WindowError,
    model::{args::ArgsSet, window_data::WindowData},
    stat::create_stat,
};
use std::collections::VecDeque;

pub async fn count_window(
    is_first_flag: &mut bool,
    stock_data_buffer: &mut VecDeque<WindowData>,
    args_set: &ArgsSet,
    window_data: WindowData,
    socket: &mut WebSocket,
) -> Result<(), WindowError> {
    // sliding window process (count based)
    if *is_first_flag {
        // first window process
        if stock_data_buffer.len() < args_set.get_window_count_value()? as usize {
            // push back WindowData
            stock_data_buffer.push_back(window_data);
        } else {
            *is_first_flag = false;
        }
        if !*is_first_flag {
            // show result
            create_stat::create_stat_data(stock_data_buffer.clone(), args_set.clone(), socket)
                .await?;
            // pop over record
            for _ in 0..args_set.get_slide_count_value()? {
                // pop first element
                stock_data_buffer.pop_front();
            }
        }
    } else {
        // other window process
        if stock_data_buffer.len() < args_set.get_window_count_value()? as usize {
            // push back WindowData
            stock_data_buffer.push_back(window_data);
        } else {
            // show result
            create_stat::create_stat_data(stock_data_buffer.clone(), args_set.clone(), socket)
                .await?;
            // pop over record
            for _ in 0..args_set.get_slide_count_value()? {
                // pop first element
                stock_data_buffer.pop_front();
            }
        }
    }
    Ok(())
}
