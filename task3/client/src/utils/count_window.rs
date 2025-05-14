use super::{args::ArgsSet, error::ClientError, model::StockData, stock_data};
use chrono::{DateTime, Utc};

pub fn count_window(
    args_set: &ArgsSet,
    is_first_flag: &mut bool,
    stock_data_buffer: &mut Vec<StockData>,
    record: Vec<String>,
) -> Result<(), ClientError> {
    // create and collect StockData
    stock_data_buffer.push(stock_data::create_stock_data(record)?);
    // update is_fisrt_flag (this flag is changed, when the first process of window was finished.)
    if *is_first_flag {
        // update is_fisrt_flag (this flag is changed, when the first process of window was finished.)
        *is_first_flag = false;
    } else {
        // after 1 windows processed.
    }
    Ok(())
}
