use super::{stat::StatData, window_data::WindowData};
use serde::Serialize;
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize)]
pub struct ResponseData {
    pub window_data: VecDeque<WindowData>,
    pub stat_data: Vec<StatData>,
}
