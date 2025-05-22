use axum::extract::ws::Utf8Bytes;

use crate::{
    error::window::WindowError,
    model::args::{ArgsSet, SlideType, SlidingWindowEnumType, WindowType},
};

pub fn create_args_set(
    text: Utf8Bytes,
    sliding_types: SlidingWindowEnumType,
) -> Result<ArgsSet, WindowError> {
    // get buffer
    let sliding_window: Vec<String> = String::from_utf8_lossy(text.as_bytes())
        .split(',')
        .map(|s| s.to_string())
        .collect();
    println!("sliding_window: {:?}", sliding_window);
    // window
    let window = match sliding_types.clone() {
        SlidingWindowEnumType::Time => match sliding_window[0].parse::<f64>() {
            Ok(window) => WindowType::Time(window),
            Err(error) => {
                return Err(WindowError::ParseFloatError(error));
            }
        },
        SlidingWindowEnumType::Count => match sliding_window[0].parse::<u64>() {
            Ok(window) => WindowType::Count(window),
            Err(error) => {
                return Err(WindowError::ParseIntError(error));
            }
        },
    };
    // slide
    let slide = match sliding_types.clone() {
        SlidingWindowEnumType::Time => match sliding_window[1].parse::<f64>() {
            Ok(slide) => SlideType::Time(slide),
            Err(error) => {
                return Err(WindowError::ParseFloatError(error));
            }
        },
        SlidingWindowEnumType::Count => match sliding_window[1].parse::<u64>() {
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
