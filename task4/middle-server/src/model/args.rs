use crate::error::window::WindowError;

#[derive(Debug, Clone)]
pub enum SlidingWindowEnumType {
    Count,
    Time,
}

#[derive(Debug, Clone)]
pub enum WindowType {
    Count(u64),
    Time(f64),
}

#[derive(Debug, Clone)]
pub enum SlideType {
    Count(u64),
    Time(f64),
}

#[derive(Debug, Clone)]
pub struct ArgsSet {
    pub types: SlidingWindowEnumType,
    pub window: WindowType,
    pub slide: SlideType,
}

impl Default for ArgsSet {
    fn default() -> Self {
        ArgsSet {
            types: SlidingWindowEnumType::Count,
            window: WindowType::Count(0),
            slide: SlideType::Count(0),
        }
    }
}

impl ArgsSet {
    pub fn get_window_count_value(&self) -> Result<u64, WindowError> {
        match self.window {
            WindowType::Count(count) => Ok(count),
            WindowType::Time(_) => Err(WindowError::GetWindowTypeCountValueError),
        }
    }
    pub fn get_window_time_value(&self) -> Result<i64, WindowError> {
        match self.window {
            WindowType::Count(_) => Err(WindowError::GetWindowTypeTimeValueError),
            WindowType::Time(time) => {
                let result = (time * 1000.0).round() as i64;
                Ok(result)
            }
        }
    }
    pub fn get_slide_count_value(&self) -> Result<u64, WindowError> {
        match self.slide {
            SlideType::Count(count) => Ok(count),
            SlideType::Time(_) => Err(WindowError::GetSlideTypeCountValueError),
        }
    }
    pub fn get_slide_time_value(&self) -> Result<i64, WindowError> {
        match self.slide {
            SlideType::Count(_) => Err(WindowError::GetSlideTypeTimeValueError),
            SlideType::Time(time) => {
                let result = (time * 1000.0).round() as i64;
                Ok(result)
            }
        }
    }
}
