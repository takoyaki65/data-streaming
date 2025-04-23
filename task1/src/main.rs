use error::GenerateMockError;
use model::StockData;
use rand::Rng;
use std::fs::File;
use std::io::Write;

pub mod error;
pub mod model;

fn main() -> Result<(), GenerateMockError> {
    // create mock data
    let mut rng = rand::rng();
    let mut mock_data: Vec<StockData> = Vec::new();
    // generate timestamp
    let mut timestamp = chrono::Utc::now().naive_utc();
    for i in 0..10_000 {
        // generate stock type
        let stock = match rng.random_range(0..26) {
            0 => model::StockEnum::StockA,
            1 => model::StockEnum::StockB,
            2 => model::StockEnum::StockC,
            3 => model::StockEnum::StockD,
            4 => model::StockEnum::StockE,
            5 => model::StockEnum::StockF,
            6 => model::StockEnum::StockG,
            7 => model::StockEnum::StockH,
            8 => model::StockEnum::StockI,
            9 => model::StockEnum::StockJ,
            10 => model::StockEnum::StockK,
            11 => model::StockEnum::StockL,
            12 => model::StockEnum::StockM,
            13 => model::StockEnum::StockN,
            14 => model::StockEnum::StockO,
            15 => model::StockEnum::StockP,
            16 => model::StockEnum::StockQ,
            17 => model::StockEnum::StockR,
            18 => model::StockEnum::StockS,
            19 => model::StockEnum::StockT,
            20 => model::StockEnum::StockU,
            21 => model::StockEnum::StockV,
            22 => model::StockEnum::StockW,
            23 => model::StockEnum::StockX,
            24 => model::StockEnum::StockY,
            _ => model::StockEnum::StockZ,
        };
        // generate open, high, low, close
        let mut rand_array = [0.0; 4];
        for value in &mut rand_array {
            *value = rng.random_range(100..100000) as f64 / 100.0;
        }
        rand_array.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // generate timestamp
        timestamp += chrono::Duration::milliseconds(i * 500);
        // push stock data to vector
        mock_data.push(StockData {
            stock,
            open: rand_array[1],
            high: rand_array[3],
            low: rand_array[0],
            close: rand_array[2],
            timestamp,
        });
    }
    // create file
    let mut file = File::create("stock_data.txt")?;
    write!(file, "stock,open,high,low,close,timestamp")?;
    mock_data.iter().try_for_each(|stock_data| {
        writeln!(
            file,
            "{},{},{},{},{},{}",
            stock_data.stock,
            stock_data.open,
            stock_data.high,
            stock_data.low,
            stock_data.close,
            stock_data.timestamp
        )
    })?;
    Ok(())
}
