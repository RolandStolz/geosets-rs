#![allow(unused)]
use super::*;
use thiserror::Error;

#[derive(Clone, Debug)]
#[allow(non_snake_case)]
pub struct Interval {
    lb: Array1<f64>,
    ub: Array1<f64>,
}

#[derive(Error, Debug)]
pub enum IntervalError {
    #[error("Dimensions of lb {lb_dim:?} and ub {ub_dim:?} do not match")]
    DimensionMismatch { lb_dim: usize, ub_dim: usize },
    #[error("Lower bound {lb} must be less than or equal to upper bound {ub}")]
    InvalidBounds { lb: Array1<f64>, ub: Array1<f64> },
}

#[allow(non_snake_case)]
impl Interval {
    pub fn new(lb: Array1<f64>, ub: Array1<f64>) -> Result<Interval, IntervalError> {
        if lb.dim() != ub.dim() {
            return Err(IntervalError::DimensionMismatch {
                lb_dim: lb.dim(),
                ub_dim: ub.dim(),
            });
        }

        if lb.iter().zip(ub.iter()).any(|(a, b)| a > b) {
            return Err(IntervalError::InvalidBounds { lb: lb, ub: ub });
        }

        Ok(Interval { lb: lb, ub: ub })
    }
}

#[allow(non_snake_case)]
impl GeoSet for Interval {
    fn from_unit_box(dim: usize) -> Self {
        Interval {
            lb: -Array::ones(dim),
            ub: Array::ones(dim),
        }
    }

    fn dim(&self) -> usize {
        self.lb.dim()
    }

    fn empty(&self) -> Result<bool, SetOperationError> {
        Ok(false)
    }

    fn to_vertices(&self) -> Result<Array2<f64>, SetOperationError> {
        todo!()
    }

    fn minkowski_sum(&self, other: &Self) -> Result<Self, SetOperationError> {
        todo!()
    }

    fn matmul(&self, mat: &Array2<f64>) -> Result<Self, SetOperationError> {
        todo!()
    }

    fn center(&self) -> Result<Array1<f64>, SetOperationError> {
        todo!()
    }

    fn support_function(&self) -> Result<(Array1<f64>, f64), SetOperationError> {
        todo!()
    }

    fn volume(&self) -> Result<f64, SetOperationError> {
        todo!()
    }

    fn minkowski_sum_(&self, other: &Self) -> Result<(), SetOperationError> {
        todo!()
    }

    fn matmul_(&self, mat: &Array2<f64>) -> Result<(), SetOperationError> {
        todo!()
    }

    fn translate(&self, vector: &Array1<f64>) -> Result<Self, SetOperationError> {
        todo!()
    }

    fn translate_(&self, vector: &Array1<f64>) -> Result<(), SetOperationError> {
        todo!()
    }
}
