#![allow(unused)]
use super::*;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::{Exp1, Uniform};
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
            return Err(IntervalError::InvalidBounds { lb, ub });
        }

        Ok(Interval { lb, ub })
    }

    pub fn from_random(dim: usize) -> Result<Interval, IntervalError> {
        let lb = Array1::random(dim, Uniform::new(-1.0, 0.0));
        let ub = Array1::random(dim, Uniform::new(0.0, 1.0));

        Interval::new(lb, ub)
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
        let mut vertices = Array2::zeros((1 << self.dim(), self.dim()));
        for i in 0..(1 << self.dim()) {
            for j in 0..self.dim() {
                vertices[[i, j]] = if (i & (1 << j)) != 0 {
                    self.ub[j]
                } else {
                    self.lb[j]
                };
            }
        }
        Ok(vertices)
    }

    fn center(&self) -> Result<Array1<f64>, SetOperationError> {
        let center = (&self.lb + &self.ub) / 2.;
        Ok(center)
    }

    fn support_function(
        &self,
        direction: Array1<f64>,
    ) -> Result<(Array1<f64>, f64), SetOperationError> {
        // For each dimension, pick ub if direction > 0, else lb
        let support_vector = self
            .lb
            .iter()
            .zip(self.ub.iter())
            .zip(direction.iter())
            .map(|((&lb, &ub), &d)| if d > 0.0 { ub } else { lb })
            .collect::<Array1<f64>>();

        let support_value = support_vector.dot(&direction);
        Ok((support_vector, support_value))
    }

    fn volume(&self) -> Result<f64, SetOperationError> {
        if self.degenerate() {
            return Ok(0.0);
        }

        let volume = self
            .lb
            .iter()
            .zip(self.ub.iter())
            .map(|(lb, ub)| ub - lb)
            .product();
        Ok(volume)
    }

    fn minkowski_sum_(&mut self, other: &Self) -> Result<(), SetOperationError> {
        self.lb += &other.lb;
        self.ub += &other.ub;
        Ok(())
    }

    fn matmul_(&mut self, mat: &Array2<f64>) -> Result<(), SetOperationError> {
        self._check_operand_dim(mat.dim().0);

        let mat_lb = mat.dot(&self.lb);
        let mat_ub = mat.dot(&self.ub);

        // Create arrays for positive and negative parts of the matrix
        let mat_pos = mat.mapv(|x| x.max(0.0));
        let mat_neg = mat.mapv(|x| x.min(0.0));

        // For positive matrix elements: pos * [lb, ub] = [pos*lb, pos*ub]
        // For negative matrix elements: neg * [lb, ub] = [neg*ub, neg*lb]
        self.lb = mat_pos.dot(&self.lb) + mat_neg.dot(&self.ub);
        self.ub = mat_pos.dot(&self.ub) + mat_neg.dot(&self.lb);

        Ok(())
    }

    fn translate_(&mut self, vector: &Array1<f64>) -> Result<(), SetOperationError> {
        self._check_operand_dim(vector.dim());
        self.lb += vector;
        self.ub += vector;
        Ok(())
    }

    fn degenerate(&self) -> bool {
        self.lb
            .iter()
            .zip(self.ub.iter())
            .any(|(lb, ub)| (ub - lb).abs() < 1e-9)
    }
}
