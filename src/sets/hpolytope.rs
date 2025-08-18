#![allow(unused)]
use super::*;
use crate::linalg_utils::rank;
use good_lp::{default_solver, variable, variables, Expression, SolverModel};
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
}

#[allow(non_snake_case)]
impl GeoSet for HPolytope {
    fn from_unit_box(dim: usize) -> Self {
        let A = ndarray::concatenate(
            Axis(0),
            &[Array2::eye(dim).view(), (-Array2::eye(dim)).view()],
        )
        .unwrap();
        let b = Array1::zeros(dim * 2);
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
