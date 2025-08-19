use thiserror::Error;

use crate::geometric_operations::ConvexHullError;

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

    #[error("Optimization failed: {source}")]
    InfeasibleOptimization {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    // #[error("Convex hull computation failed: {source}")]
    // ConvexHullError {
    //     #[source]
    //     source: Box<dyn std::error::Error>,
    // },

    #[error("ConvexHull error: {0}")]
    QhullError(#[from] ConvexHullError),

    #[error("Set is empty")]
    EmptySet,
}
