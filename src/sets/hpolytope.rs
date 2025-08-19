#![allow(unused)]
use super::*;
use crate::cddlib_rs::compute_polytope_vertices;
use crate::linalg_utils::rank;
use crate::VPolytope;
use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel};
use ndarray_linalg::Norm;
use ndarray_rand::rand_distr::{Normal, StandardNormal, Uniform};
use ndarray_rand::RandomExt;
use plotly::box_plot;
use thiserror::Error;

#[derive(Clone, Debug)]
#[allow(non_snake_case)]
pub struct HPolytope {
    A: Array2<f64>,
    b: Array1<f64>,
}

#[derive(Error, Debug)]
pub enum HPolytopeError {
    #[error("Dimensions of A {a_dim:?} and b {b_dim:?} do not match")]
    DimensionMismatch { a_dim: (usize, usize), b_dim: usize },
}

#[allow(non_snake_case)]
impl HPolytope {
    pub fn new(A: Array2<f64>, b: Array1<f64>) -> Result<HPolytope, HPolytopeError> {
        if A.dim().0 != b.dim() {
            Err(HPolytopeError::DimensionMismatch {
                a_dim: A.dim(),
                b_dim: b.dim(),
            })
        } else {
            Ok(HPolytope { A, b })
        }
    }

    pub fn from_random(dim: usize, n_constraints: usize) -> Result<HPolytope, HPolytopeError> {
        let box_poly = HPolytope::from_unit_box(dim);

        let mut random_A = Array2::random((n_constraints, dim), StandardNormal);
        // Normalize random_A
        for mut row in random_A.rows_mut() {
            let norm = row.norm_l2();
            if norm > 0.0 {
                row /= norm;
            }
        }

        let interior_point = Array1::random(dim, Uniform::new(-0.8, 0.8));
        let offsets = Array1::random(n_constraints, Uniform::new(0.1, 1.0));

        let random_b = random_A.dot(&interior_point) + offsets;

        let A = ndarray::concatenate(Axis(0), &[box_poly.A.view(), random_A.view()]).unwrap();
        let b = ndarray::concatenate(Axis(0), &[box_poly.b.view(), random_b.view()]).unwrap();

        Ok(HPolytope { A, b })
    }

    pub fn n_constraints(&self) -> usize {
        self.A.nrows()
    }
}

#[allow(non_snake_case)]
impl GeoSet for HPolytope {
    fn from_unit_box(dim: usize) -> Self {
        let A = ndarray::concatenate(
            Axis(0),
            &[Array2::eye(dim).view(), (-Array2::eye(dim)).view()],
        )
        .unwrap();
        let b = Array1::ones(dim * 2);
        HPolytope::new(A, b).unwrap()
    }

    fn dim(&self) -> usize {
        self.A.dim().1
    }

    fn empty(&self) -> Result<bool, SetOperationError> {
        let m = self.A.nrows();
        let n = self.A.ncols();

        // Define variables x_0, ..., x_{n-1} (unbounded)
        let mut vars = variables!();
        let x: Vec<_> = (0..n)
            .map(|_| vars.add(variable().bounds(f64::NEG_INFINITY..f64::INFINITY)))
            .collect();

        // Build the problem with dummy objective
        let mut problem = vars.minimise(0.0).using(default_solver);

        // Add constraints row by row: A[i] ⋅ x <= b[i]
        for i in 0..m {
            let row = self.A.row(i);
            let expr: Expression = row.iter().zip(&x).map(|(coef, xi)| *coef * *xi).sum();
            problem = problem.with(expr.leq(self.b[i]));
        }

        // Try solving
        match problem.solve() {
            Ok(_) => Ok(false), // feasible → not empty
            Err(_) => Ok(true), // infeasible → empty
        }
    }

    fn to_vertices(&self) -> Result<Array2<f64>, SetOperationError> {
        let empty = self.empty()?;
        if empty {
            return Err(SetOperationError::EmptySet);
        }
        compute_polytope_vertices(&self.A, &self.b)
    }

    fn center(&self) -> Result<Array1<f64>, SetOperationError> {
        let mut vars = variables!();
        let r = vars.add(variable().min(0.0)); // radius >= 0
        let x: Vec<_> = (0..self.dim()).map(|_| vars.add(variable())).collect();

        let mut problem = vars.maximise(r).using(default_solver);

        for i in 0..self.n_constraints() {
            let row = self.A.row(i);
            let norm_ai = row.dot(&row).sqrt();

            // let
            let expr: Expression = row
                .iter()
                .zip(&x)
                .map(|(&aij, &xj)| aij * xj)
                .sum::<Expression>()
                + norm_ai * r;

            problem = problem.with(expr.leq(self.b[i]));
        }

        let solution = problem
            .solve()
            .map_err(|e| SetOperationError::InfeasibleOptimization {
                source: Box::new(e),
            })?;

        let center =
            Array1::from_shape_vec(self.dim(), x.iter().map(|&xi| solution.value(xi)).collect())
                .map_err(|e| SetOperationError::InfeasibleOptimization {
                    source: Box::new(e),
                })?;

        Ok(center)
    }

    fn support_function(&self) -> Result<(Array1<f64>, f64), SetOperationError> {
        todo!()
    }

    fn volume(&self) -> Result<f64, SetOperationError> {
        let vpoly = VPolytope::new(self.to_vertices()?).map_err(|_| SetOperationError::EmptySet)?;
        vpoly.volume()
    }

    fn minkowski_sum_(&mut self, other: &Self) -> Result<(), SetOperationError> {
        todo!()
    }

    fn matmul_(&mut self, mat: &Array2<f64>) -> Result<(), SetOperationError> {
        let (m, n) = mat.dim();
        self._check_operand_dim(m)?;
        let mat_rank = rank(mat).unwrap();

        if m == n && mat_rank == n {
            self.A = self.A.dot(mat);
        } else if m > n {
            return Err(SetOperationError::NotImplemented);
        }
        Ok(())
    }

    fn translate_(&mut self, vector: &Array1<f64>) -> Result<(), SetOperationError> {
        self._check_operand_dim(vector.dim())?;
        self.b = &self.b + &self.A.dot(vector);
        Ok(())
    }

    fn degenerate(&self) -> bool {
        let c = match self.center() {
            Ok(center) => center,
            Err(SetOperationError::InfeasibleOptimization { .. }) => {
                // Empty sets are degenerate
                return true;
            }
            Err(_) => {
                // Other errors: treat as degenerate (or propagate)
                return true;
            }
        };

        let residual = &self.b - &self.A.dot(&c);

        // Check if any inequality is tight (≈ equality)
        residual.iter().any(|&x| (x - 0.0).abs() <= 1e-9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polytope_new() {
        let _ = HPolytope::new(Array::ones((2, 2)), Array::ones(2)).unwrap();
        let _ = HPolytope::new(Array::ones((5, 2)), Array::ones(5)).unwrap();
    }
}
