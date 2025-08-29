use thiserror::Error;

use crate::qhull_wrapper::ConvexHullError;

#[derive(Error, Debug)]
pub enum SetOperationError {
    #[error("Dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },

    #[error("Operation not implemented")]
    NotImplemented,

    #[error("Data conversion error: {source}")]
    DataConversionError {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Unsupported operation, {message}")]
    UnsupportedOperation { message: String },

    #[error("Optimization failed: {source}")]
    InfeasibleOptimization {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("ConvexHull error: {0}")]
    QhullError(#[from] ConvexHullError),

    #[error("Set is empty")]
    EmptySet,
}
