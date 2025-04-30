use crate::error::ClientError;

#[derive(Debug, Clone)]
pub enum StockEnum {
    StockA,
    StockB,
    StockC,
    StockD,
    StockE,
    StockF,
    StockG,
    StockH,
    StockI,
    StockJ,
    StockK,
    StockL,
    StockM,
    StockN,
    StockO,
    StockP,
    StockQ,
    StockR,
    StockS,
    StockT,
    StockU,
    StockV,
    StockW,
    StockX,
    StockY,
    StockZ,
}

impl std::fmt::Display for StockEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StockEnum::StockA => write!(f, "StockA"),
            StockEnum::StockB => write!(f, "StockB"),
            StockEnum::StockC => write!(f, "StockC"),
            StockEnum::StockD => write!(f, "StockD"),
            StockEnum::StockE => write!(f, "StockE"),
            StockEnum::StockF => write!(f, "StockF"),
            StockEnum::StockG => write!(f, "StockG"),
            StockEnum::StockH => write!(f, "StockH"),
            StockEnum::StockI => write!(f, "StockI"),
            StockEnum::StockJ => write!(f, "StockJ"),
            StockEnum::StockK => write!(f, "StockK"),
            StockEnum::StockL => write!(f, "StockL"),
            StockEnum::StockM => write!(f, "StockM"),
            StockEnum::StockN => write!(f, "StockN"),
            StockEnum::StockO => write!(f, "StockO"),
            StockEnum::StockP => write!(f, "StockP"),
            StockEnum::StockQ => write!(f, "StockQ"),
            StockEnum::StockR => write!(f, "StockR"),
            StockEnum::StockS => write!(f, "StockS"),
            StockEnum::StockT => write!(f, "StockT"),
            StockEnum::StockU => write!(f, "StockU"),
            StockEnum::StockV => write!(f, "StockV"),
            StockEnum::StockW => write!(f, "StockW"),
            StockEnum::StockX => write!(f, "StockX"),
            StockEnum::StockY => write!(f, "StockY"),
            StockEnum::StockZ => write!(f, "StockZ"),
        }
    }
}

impl std::str::FromStr for StockEnum {
    type Err = ClientError;
    fn from_str(s: &str) -> Result<Self, ClientError> {
        match s {
            "StockA" => Ok(StockEnum::StockA),
            "StockB" => Ok(StockEnum::StockB),
            "StockC" => Ok(StockEnum::StockC),
            "StockD" => Ok(StockEnum::StockD),
            "StockE" => Ok(StockEnum::StockE),
            "StockF" => Ok(StockEnum::StockF),
            "StockG" => Ok(StockEnum::StockG),
            "StockH" => Ok(StockEnum::StockH),
            "StockI" => Ok(StockEnum::StockI),
            "StockJ" => Ok(StockEnum::StockJ),
            "StockK" => Ok(StockEnum::StockK),
            "StockL" => Ok(StockEnum::StockL),
            "StockM" => Ok(StockEnum::StockM),
            "StockN" => Ok(StockEnum::StockN),
            "StockO" => Ok(StockEnum::StockO),
            "StockP" => Ok(StockEnum::StockP),
            "StockQ" => Ok(StockEnum::StockQ),
            "StockR" => Ok(StockEnum::StockR),
            "StockS" => Ok(StockEnum::StockS),
            "StockT" => Ok(StockEnum::StockT),
            "StockU" => Ok(StockEnum::StockU),
            "StockV" => Ok(StockEnum::StockV),
            "StockW" => Ok(StockEnum::StockW),
            "StockX" => Ok(StockEnum::StockX),
            "StockY" => Ok(StockEnum::StockY),
            "StockZ" => Ok(StockEnum::StockZ),
            other => Err(ClientError::StockEnumParseError(other.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StockData {
    pub stock: StockEnum,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub timestamp: chrono::NaiveDateTime,
}
