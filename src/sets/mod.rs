use ndarray::prelude::*;
use ndarray::concatenate;

use self::errors::{PolytopeError, ZonotopeError, SetOperationError};

mod errors;
mod operations;
mod polytope;
mod zonotope;

trait GeoSet: Sized {
    fn dim(&self) -> usize;
    fn to_vertices(&self) -> Array2<f64>;

    fn minkowski_sum(&self, other: &Self) -> Result<Self, SetOperationError>;
    fn linear_transform(&self, mat: &Array2<f64>) -> Result<Self, SetOperationError>;
    fn scalar_product(&self, scalar: f64) -> Result<Self, SetOperationError>;
}
