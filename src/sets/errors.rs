use std::fmt;

#[derive(Debug)]
pub enum SetOperationError {
    DimensionMismatch,
}

#[derive(Debug)]
pub enum ZonotopeError {
    DimensionMismatch,
}

#[derive(Debug)]
pub enum PolytopeError {}

impl fmt::Display for ZonotopeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZonotopeError::DimensionMismatch => write!(f, "Dimensions of G and c do not match!"),
        }
    }
}

impl fmt::Display for SetOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SetOperationError::DimensionMismatch => {
                write!(f, "Dimensions of the two operators do not match!")
            }
        }
    }
}
