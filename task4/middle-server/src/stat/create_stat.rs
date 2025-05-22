use crate::{
    error::window::WindowError,
    model::{
        args::ArgsSet, response::ResponseData, stat::StatData, stock_data::StockEnum,
        window_data::WindowData,
    },
    stat::show,
};
use axum::extract::ws::{Message, WebSocket};
use statrs::statistics::Statistics;
use std::collections::{HashMap, VecDeque};

pub async fn create_stat_data(
    stock_data_buffer: VecDeque<WindowData>,
    args_set: ArgsSet,
    socket: &mut WebSocket,
) -> Result<(), WindowError> {
    //* create StatData *//
    // init stat_data
    let mut stat_data: Vec<StatData> = Vec::new();
    // edit stock_data
    let mut hash_map: HashMap<StockEnum, Vec<f64>> = HashMap::new();
    for v in stock_data_buffer.iter() {
        let stock_kind: StockEnum = v.stock_data.stock.clone();
        let value: &mut Vec<f64> = hash_map.entry(stock_kind).or_default();
        value.push(v.stock_data.close);
    }
    let mut stocks: Vec<(String, Vec<f64>)> = hash_map
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect::<Vec<(String, Vec<f64>)>>();
    stocks.sort_by(|(key_a, _), (key_b, _)| key_a.cmp(key_b));
    stocks.clone().into_iter().for_each(|(key, value)| {
        stat_data.push(StatData {
            stock: key.clone(),
            max: value.clone().max(),
            min: value.clone().min(),
            mean: value.clone().mean(),
            std_dev: value.clone().std_dev(),
        });
    });

    //* show result *//
    show::show_stat_result(stock_data_buffer.clone(), args_set, stocks)?;

    //* send result to client *//
    // create response data
    let result = serde_json::to_string(
        &(ResponseData {
            window_data: stock_data_buffer,
            stat_data,
        }),
    )?;
    // send result to client
    (*socket).send(Message::Text(result.into())).await?;

    Ok(())
}
