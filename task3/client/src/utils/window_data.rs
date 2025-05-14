use super::stock_data::StockData;

#[derive(Debug, Clone)]
pub struct WindowData {
    pub stock_data: StockData,
    pub timestamp: chrono::NaiveDateTime,
}

impl WindowData {
    pub fn new(stock_data: StockData) -> Self {
        WindowData {
            stock_data,
            timestamp: chrono::Utc::now().naive_utc(),
        }
    }
    pub fn get_timestamp(&self) -> chrono::NaiveDateTime {
        self.timestamp
    }
}
