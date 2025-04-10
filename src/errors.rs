#[derive(Debug)]
pub enum CalculatorError {
    InvalidNumber(String),
    WrongFormat(String),
}

impl std::fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CalculatorError::InvalidNumber(s) => write!(f, "Zła liczba, kurwa: {}", s),
            CalculatorError::WrongFormat(s) => write!(f, "Zły format, kurwa: {}", s),
        }
    }
}

impl std::error::Error for CalculatorError {}
