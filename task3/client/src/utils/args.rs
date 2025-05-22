use crate::error::window::WindowError;

#[derive(Debug, Clone)]
pub enum SlidingWindowEnumType {
    Count,
    Time,
}

impl std::str::FromStr for SlidingWindowEnumType {
    type Err = WindowError;
    fn from_str(s: &str) -> Result<Self, WindowError> {
        match s {
            "--count" => Ok(SlidingWindowEnumType::Count),
            "--time" => Ok(SlidingWindowEnumType::Time),
            _ => Err(WindowError::ParseArgsError(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WindowType {
    Count(u64),
    Time(f64),
}

#[derive(Debug, Clone)]
pub enum CountType {
    Count(u64),
    Time(f64),
}

#[derive(Debug, Clone)]
pub struct ArgsSet {
    pub types: SlidingWindowEnumType,
    pub window: WindowType,
    pub count: CountType,
}

impl Default for ArgsSet {
    fn default() -> Self {
        let types = SlidingWindowEnumType::Count;
        let window = WindowType::Count(0);
        let count = CountType::Count(0);
        ArgsSet {
            types,
            window,
            count,
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
        match self.count {
            CountType::Count(count) => Ok(count),
            CountType::Time(_) => Err(WindowError::GetSlideTypeCountValueError),
        }
    }
    pub fn get_slide_time_value(&self) -> Result<i64, WindowError> {
        match self.count {
            CountType::Count(_) => Err(WindowError::GetSlideTypeTimeValueError),
            CountType::Time(time) => {
                let result = (time * 1000.0).round() as i64;
                Ok(result)
            }
        }
    }
}

pub fn parse_args(args: Vec<String>) -> Result<ArgsSet, WindowError> {
    let mut args_set = ArgsSet::default();
    for i in 0..args.len() {
        if i == 1 && (args[1] == "--count" || args[1] == "--time") {
            if args[1] == "--count" {
                args_set.types = SlidingWindowEnumType::Count;
            } else if args[1] == "--time" {
                args_set.types = SlidingWindowEnumType::Time;
            }
        } else if i == 2 && (args[2] == "--window") {
            if args[1] == "--count" {
                args_set.window = WindowType::Count(args[3].parse::<u64>()?);
            } else if args[1] == "--time" {
                args_set.window = WindowType::Time(args[3].parse::<f64>()?);
            }
        } else if i == 4 && (args[4] == "--slide") {
            if args[1] == "--count" {
                args_set.count = CountType::Count(args[5].parse::<u64>()?);
            } else if args[1] == "--time" {
                args_set.count = CountType::Time(args[5].parse::<f64>()?);
            }
        } else if i == 0 || i == 3 || i == 5 {
            continue;
        } else {
            return Err(WindowError::ParseArgsError(
                "Invalid argument pattern. Use cargo run -- --(count|time) --window (window size as number (window > slide)) --slide (slide size as number (window > slide)).".to_string(),
            ));
        }
    }

    Ok(args_set)
}
