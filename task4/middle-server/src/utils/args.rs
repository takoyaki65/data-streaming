use crate::{
    error::window::WindowError,
    model::args::{ArgsSet, SlideType, SlidingWindowEnumType, WindowType},
};
use axum::extract::ws::Utf8Bytes;

pub fn create_args_set(text: Utf8Bytes) -> Result<ArgsSet, WindowError> {
    // get buffer
    let sliding_window: Vec<String> = String::from_utf8_lossy(text.as_bytes())
        .split(',')
        .map(|s| s.to_string())
        .collect();
    println!("sliding_window: {:?}", sliding_window);
    // sliding type
    let sliding_types: SlidingWindowEnumType = match sliding_window[0].as_str() {
        "time" => SlidingWindowEnumType::Time,
        "count" => SlidingWindowEnumType::Count,
        _ => {
            return Err(WindowError::InvalidSlidingWindowType(
                sliding_window[0].clone(),
            ));
        }
    };
    // window
    let window = match sliding_types.clone() {
        SlidingWindowEnumType::Time => match sliding_window[1].parse::<f64>() {
            Ok(window) => WindowType::Time(window),
            Err(error) => {
                return Err(WindowError::ParseFloatError(error));
            }
        },
        SlidingWindowEnumType::Count => match sliding_window[1].parse::<u64>() {
            Ok(window) => WindowType::Count(window),
            Err(error) => {
                return Err(WindowError::ParseIntError(error));
            }
        },
    };
    // slide
    let slide = match sliding_types.clone() {
        SlidingWindowEnumType::Time => match sliding_window[2].parse::<f64>() {
            Ok(slide) => SlideType::Time(slide),
            Err(error) => {
                return Err(WindowError::ParseFloatError(error));
            }
        },
        SlidingWindowEnumType::Count => match sliding_window[2].parse::<u64>() {
            Ok(slide) => SlideType::Count(slide),
            Err(error) => {
                return Err(WindowError::ParseIntError(error));
            }
        },
    };
    // create a args_set
    Ok(ArgsSet {
        types: sliding_types.clone(),
        window,
        slide,
    })
}
