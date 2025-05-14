use crate::{
    utils::error::ClientError,
    utils::model::{StockData, StockEnum},
};

pub fn create_stock_data(record: Vec<String>) -> Result<StockData, ClientError> {
    // create StockData object
    let stock = record[0].parse::<StockEnum>()?;
    let open = record[1].parse::<f64>()?;
    let high = record[2].parse::<f64>()?;
    let low = record[3].parse::<f64>()?;
    let close = record[4].parse::<f64>()?;
    let timestamp =
        chrono::NaiveDateTime::parse_from_str(record[5].as_str(), "%Y-%m-%d %H:%M:%S.%f")?;
    Ok(StockData {
        stock,
        open,
        high,
        low,
        close,
        timestamp,
    })
}
