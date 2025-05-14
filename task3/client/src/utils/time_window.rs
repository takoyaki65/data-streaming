use super::{args::ArgsSet, error::ClientError, model::StockData, stock_data};
use chrono::{DateTime, Utc};

pub fn time_window(
    args_set: &ArgsSet,
    is_first_flag: &mut bool,
    start_time: &mut DateTime<Utc>,
    current_time: &mut DateTime<Utc>,
    stock_data_buffer: &mut Vec<StockData>,
    record: Vec<String>,
) -> Result<(), ClientError> {
    // create and collect StockData
    stock_data_buffer.push(stock_data::create_stock_data(record)?);
    // update is_fisrt_flag (this flag is changed, when the first process of window was finished.)
    if *is_first_flag {
        // first process
        *is_first_flag = false;
    } else {
        // other process
    }
    Ok(())
}
