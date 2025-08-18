use ndarray::concatenate;
use ndarray::prelude::*;

use self::errors::SetOperationError;

pub mod errors;
pub mod hpolytope;
pub mod interval;
pub mod vpolytope;
pub mod zonotope;

// TODO: figure out how you can use the specific set error in the result!
pub trait GeoSet: Sized {
    fn dim(&self) -> usize;
    fn empty(&self) -> Result<bool, SetOperationError>;

    // Static function
    fn from_unit_box(dim: usize) -> Self;

    fn to_vertices(&self) -> Result<Array2<f64>, SetOperationError>;
    fn center(&self) -> Result<Array1<f64>, SetOperationError>;
    fn support_function(&self) -> Result<(Array1<f64>, f64), SetOperationError>;
    fn volume(&self) -> Result<f64, SetOperationError>;
    fn plot(&self) -> Result<(), SetOperationError>;

    // TODO: Can I implement the minkowski_sum_ generic for all?
    fn minkowski_sum(&self, other: &Self) -> Result<Self, SetOperationError>;
    fn minkowski_sum_(&self, other: &Self) -> Result<(), SetOperationError>;

    fn matmul(&self, mat: &Array2<f64>) -> Result<Self, SetOperationError>;
    fn matmul_(&self, mat: &Array2<f64>) -> Result<(), SetOperationError>;

    fn translate(&self, vector: &Array1<f64>) -> Result<Self, SetOperationError>;
    fn translate_(&self, vector: &Array1<f64>) -> Result<(), SetOperationError>;
}
