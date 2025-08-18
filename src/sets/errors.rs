use thiserror::Error;

#[derive(Error, Debug)]
pub enum SetOperationError {
    #[error("Dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },
}
