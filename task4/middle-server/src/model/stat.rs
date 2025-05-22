use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StatData {
    pub stock: String,
    pub max: f64,
    pub min: f64,
    pub mean: f64,
    pub std_dev: f64,
}
