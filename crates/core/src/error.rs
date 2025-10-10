use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum GeometryError {
    InvalidConstruction(String),
    DegenerateCase(String),
    NoIntersection(String),
    InvalidParameter(String),
    DivisionByZero(String),
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeometryError::InvalidConstruction(msg) => {
                write!(f, "Invalid geometric construction: {}", msg)
            }
            GeometryError::DegenerateCase(msg) => {
                write!(f, "Degenerate geometry: {}", msg)
            }
            GeometryError::NoIntersection(msg) => {
                write!(f, "No intersection found: {}", msg)
            }
            GeometryError::InvalidParameter(msg) => {
                write!(f, "Invalid parameter: {}", msg)
            }
            GeometryError::DivisionByZero(msg) => {
                write!(f, "Division by zero: {}", msg)
            }
        }
    }
}

impl std::error::Error for GeometryError {}

pub type Result<T> = std::result::Result<T, GeometryError>;
