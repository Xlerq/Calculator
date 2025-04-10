#[derive(Debug)]
pub enum CalculatorError {
    InvalidNumber(String),
    WrongFormat(String),
    DivisionByZero,
}

impl std::fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CalculatorError::InvalidNumber(s) => write!(f, "Zła liczba, kurwa: {}", s),
            CalculatorError::WrongFormat(s) => write!(f, "Zły format, kurwa: {}", s),
            CalculatorError::DivisionByZero => write!(f, "Dzielenie przez zero, kurwa!"),
        }
    }
}

impl std::error::Error for CalculatorError {}
