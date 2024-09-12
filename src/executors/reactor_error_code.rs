pub enum ReactorErrorCode {
    InvalidDeadline,
    OrderNotFillable,
    OrderAlreadyFilled,
    Unknown,
}

// implements the From trait for the ReactorErrorCode enum to convert it to a string
impl From<String> for ReactorErrorCode {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0xc6035520" => ReactorErrorCode::OrderNotFillable,
            "0xee3b3d4b" => ReactorErrorCode::OrderAlreadyFilled,
            "0x769d11e4" => ReactorErrorCode::InvalidDeadline,
            _ => ReactorErrorCode::Unknown,
        }
    }
}

impl std::fmt::Display for ReactorErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ReactorErrorCode::InvalidDeadline => "InvalidDeadline",
            ReactorErrorCode::OrderNotFillable => "OrderNotFillable",
            ReactorErrorCode::OrderAlreadyFilled => "OrderAlreadyFilled",
            ReactorErrorCode::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}
