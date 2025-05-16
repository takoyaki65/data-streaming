use super::{
    args::ArgsSet,
    error::ClientError,
    stat::{self},
    window_data::WindowData,
};
use std::collections::VecDeque;

pub fn time_window(
    is_first_flag: &mut bool,
    stock_data_buffer: &mut VecDeque<WindowData>,
    args_set: &ArgsSet,
    window_data: WindowData,
) -> Result<(), ClientError> {
    // check if the time is over
    if *is_first_flag {
        // first window process
        if stock_data_buffer.is_empty() {
            stock_data_buffer.push_back(window_data);
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
            let time_diff = window_data.get_timestamp() - first_element.get_timestamp();
            // check if the time is over
            if time_diff.num_milliseconds() < args_set.get_window_time_value()? {
                // push back WindowData
                stock_data_buffer.push_back(window_data);
            } else {
                *is_first_flag = false;
            }
            if !*is_first_flag {
                // show result
                stat::show_stat(stock_data_buffer.clone(), args_set.clone())?;
                // pop over record
                let pop_slide_time = first_element.get_timestamp()
                    + chrono::Duration::milliseconds(args_set.get_slide_time_value()?);
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
            < chrono::Duration::milliseconds(args_set.get_window_time_value()?)
        {
            // push back WindowData
            stock_data_buffer.push_back(window_data);
        } else {
            // show result
            stat::show_stat(stock_data_buffer.clone(), args_set.clone())?;
            // pop over record
            let pop_slide_time = first_element.get_timestamp()
                + chrono::Duration::milliseconds(args_set.get_slide_time_value()?);
            while pop_slide_time - stock_data_buffer[0].get_timestamp() > chrono::Duration::zero() {
                // pop front element
                stock_data_buffer.pop_front();
                if stock_data_buffer.is_empty() {
                    break;
                }
            }
        }
    }
    Ok(())
}
