#![allow(unused)]
use crate::geometric_operations::convex_hull;

use super::*;
use ndarray_rand::rand_distr::{Exp1, Uniform};
use ndarray_rand::RandomExt;
use plotly::common::Mode;
use plotly::{Plot, Scatter};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct VPolytope {
    vertices: Array2<f64>,
}

#[derive(Error, Debug)]
pub enum VPolytopeError {
    #[error("Vertices must not be empty!")]
    EmptyVertices,
}

impl VPolytope {
    pub fn new(vertices: Array2<f64>) -> Result<VPolytope, VPolytopeError> {
        if vertices.is_empty() {
            return Err(VPolytopeError::EmptyVertices);
        }
        Ok(VPolytope { vertices })
    }

    pub fn from_random(dim: usize, n_vertices: usize) -> Result<VPolytope, VPolytopeError> {
        let vertices = Array2::random((n_vertices, dim), Uniform::new(-1.0, 1.0));
        VPolytope::new(vertices)
    }

    pub fn n_vertices(&self) -> usize {
        self.vertices.nrows()
    }

    pub fn compact_(&mut self) -> Result<(), SetOperationError> {
        self.vertices = convex_hull(self.vertices.clone())?;
        Ok(())
    }

    pub fn compact(&self) -> Result<VPolytope, SetOperationError> {
        let mut copy = self.clone();
        copy.compact_()?;
        Ok(copy)
    }
}

impl GeoSet for VPolytope {
    fn dim(&self) -> usize {
        self.vertices.dim().1
    }

    fn empty(&self) -> Result<bool, SetOperationError> {
        Ok(false)
    }

    fn from_unit_box(dim: usize) -> Self {
        let vertices = Array2::from_shape_fn((1 << dim, dim), |(i, j)| {
            if (i & (1 << j)) != 0 {
                1.0
            } else {
                -1.0
            }
        });
        VPolytope::new(vertices).unwrap()
    }

    fn to_vertices(&self) -> Result<Array2<f64>, SetOperationError> {
        Ok(self.compact()?.vertices)
    }

    fn center(&self) -> Result<Array1<f64>, SetOperationError> {
        // Centroid. Chebyshev center requires halfspaces
        let center = self.vertices.mean_axis(Axis(0)).unwrap();
        Ok(center)
    }

    fn support_function(&self) -> Result<(Array1<f64>, f64), SetOperationError> {
        todo!()
    }

    fn volume(&self) -> Result<f64, SetOperationError> {
        todo!()
    }

    fn minkowski_sum_(&mut self, other: &Self) -> Result<(), SetOperationError> {
        todo!()
    }

    fn matmul_(&mut self, mat: &Array2<f64>) -> Result<(), SetOperationError> {
        self._check_operand_dim(mat.dim().0);
        self.vertices = mat.dot(&self.vertices);
        Ok(())
    }

    fn translate_(&mut self, vector: &Array1<f64>) -> Result<(), SetOperationError> {
        self._check_operand_dim(vector.dim())?;
        // Translate each vertex by the vector
        self.vertices = &self.vertices + &vector.view().insert_axis(Axis(0));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polytope_new() {
        let _ = VPolytope::new(Array::ones((2, 2))).unwrap();
        let _ = VPolytope::new(Array::ones((2, 5))).unwrap();
    }
}
