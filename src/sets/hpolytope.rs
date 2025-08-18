#![allow(unused)]
use super::*;
use good_lp::{default_solver, variable, variables, Expression, ProblemVariables, SolverModel};

#[allow(non_snake_case)]
pub struct HPolytope {
    A: Array2<f64>,
    b: Array1<f64>,
}

#[allow(non_snake_case)]
impl HPolytope {
    pub fn new(A: Array2<f64>, b: Array1<f64>) -> Result<HPolytope, HPolytopeError> {
        if A.dim().0 != b.dim() {
            Err(HPolytopeError::DimensionMismatch)
        } else {
            Ok(HPolytope { A, b })
        }
    }
}

#[allow(non_snake_case)]
impl GeoSet for HPolytope {
    fn from_unit_box(dim: usize) -> Result<Self, SetOperationError> {
        let A = ndarray::concatenate(
            Axis(0),
            &[Array2::eye(dim).view(), (-Array2::eye(dim)).view()],
        )
        .unwrap();
        let b = Array1::zeros(dim * 2);
        Ok(HPolytope::new(A, b).unwrap())
    }

    fn dim(&self) -> usize {
        self.A.dim().1
    }

    fn empty(&self) -> bool {
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
            Ok(_) => false, // feasible → not empty
            Err(_) => true, // infeasible → empty
        }
    }

    fn to_vertices(&self) -> Result<Self, SetOperationError> {
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

    fn plot(&self) -> Result<(), SetOperationError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polytope_new() {
        let _ = HPolytope::new(Array::ones((2, 2)), Array::ones(2)).unwrap();
        let _ = HPolytope::new(Array::ones((5, 2)), Array::ones(5)).unwrap();
    }
}
