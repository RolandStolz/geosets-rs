#![allow(unused)]
use crate::linalg_utils::{argmax, rank};
use crate::qhull_wrapper::{convex_hull, convex_hull_vertices, qhull_volume};

use super::*;
use good_lp::{Expression, Solution, SolverModel, default_solver, variable, variables};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::{Exp1, Uniform};
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
        self.vertices = convex_hull_vertices(&self.vertices)?;
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
            if (i & (1 << j)) != 0 { 1.0 } else { -1.0 }
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

    fn support_function(
        &self,
        direction: Array1<f64>,
    ) -> Result<(Array1<f64>, f64), SetOperationError> {
        self._check_operand_dim(direction.dim())?;

        let dot_product = self.vertices.dot(&direction);
        let max_index = argmax(&dot_product).unwrap();

        let support_value = dot_product[max_index];
        let support_vector = self.vertices.row(max_index).to_owned();

        Ok((support_vector, support_value))
    }

    fn volume(&self) -> Result<f64, SetOperationError> {
        if self.degenerate() {
            return Ok(0.0);
        }

        let vertices = self.to_vertices()?;
        let qh = convex_hull(&vertices, true)?;

        Ok(qhull_volume(&qh, &vertices)?)
    }

    fn minkowski_sum_(&mut self, other: &Self) -> Result<(), SetOperationError> {
        let mut vertices = Array2::zeros((self.n_vertices() * other.n_vertices(), self.dim()));

        for (i, row_self) in self.vertices.outer_iter().enumerate() {
            for (j, row_other) in other.vertices.outer_iter().enumerate() {
                vertices
                    .row_mut(i * other.n_vertices() + j)
                    .assign(&(&row_self + &row_other));
            }
        }

        self.vertices = convex_hull_vertices(&vertices)?;
        Ok(())
    }

    fn matmul_(&mut self, mat: &Array2<f64>) -> Result<(), SetOperationError> {
        self._check_operand_dim(mat.dim().0)?;
        self.vertices = self.vertices.dot(&mat.t());
        Ok(())
    }

    fn translate_(&mut self, vector: &Array1<f64>) -> Result<(), SetOperationError> {
        self._check_operand_dim(vector.dim())?;
        // Translate each vertex by the vector
        self.vertices = &self.vertices + &vector.view().insert_axis(Axis(0));
        Ok(())
    }

    fn degenerate(&self) -> bool {
        if self.n_vertices() == 1 {
            return true;
        }
        let mat = &self.vertices - self.vertices.mean_axis(Axis(0)).unwrap();
        rank(&mat).unwrap() < self.dim()
    }

    /// Evaluates the feasibility of the optimization problem
    /// $\min 0$ \
    /// $\text{subject to } V \lambda = p; 1^\top \lambda = 1, \lambda \geq 0 b$ \
    fn contains_point(&self, point: &Array1<f64>) -> Result<bool, SetOperationError> {
        let mut vars = variables!();
        let lambda: Vec<_> = (0..self.n_vertices())
            .map(|_| vars.add(variable().min(0.0))) // \lambda \geq 0
            .collect();

        // Build the problem with dummy objective
        let mut problem = vars.minimise(0.0).using(default_solver);

        // Add constraint V \lambda = p
        for i in 0..self.dim() {
            let v = self.vertices.column(i);
            let expr: Expression = v.iter().zip(&lambda).map(|(vi, li)| *vi * *li).sum();
            problem = problem.with(expr.eq(point[i]));
        }

        // 1^\top \lambda = 1
        let expr: Expression = lambda.iter().map(|li| *li).sum();
        problem = problem.with(expr.eq(1.0));

        // Try solving
        match problem.solve() {
            Ok(_) => Ok(true),   // feasible → not empty
            Err(_) => Ok(false), // infeasible → empty
        }
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
