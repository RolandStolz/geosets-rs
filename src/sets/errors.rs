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
pub enum HPolytopeError {
    DimensionMismatch,
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

impl fmt::Display for ZonotopeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZonotopeError::DimensionMismatch => write!(f, "Dimensions of G and c do not match!"),
        }
    }
}

impl fmt::Display for HPolytopeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HPolytopeError::DimensionMismatch => write!(f, "Dimensions of A and b do not match!"),
        }
    }
}
