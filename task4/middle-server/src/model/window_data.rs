use super::stock_data::StockData;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct WindowData {
    pub stock_data: StockData,
    pub timestamp: chrono::NaiveDateTime,
    pub id: u128,
}

impl WindowData {
    pub fn new(stock_data: StockData, id: u128) -> Self {
        WindowData {
            stock_data,
            timestamp: chrono::Utc::now().naive_utc(),
            id,
        }
    }
    pub fn get_timestamp(&self) -> chrono::NaiveDateTime {
        self.timestamp
    }
    pub fn get_id(&self) -> u128 {
        self.id
    }
}
