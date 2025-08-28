#![allow(unused)]
use super::*;
use crate::VPolytope;
use crate::cddlib_rs::compute_polytope_vertices;
use crate::linalg_utils::{rank, vector_leq};
use good_lp::{Expression, Solution, SolverModel, default_solver, variable, variables};
use ndarray_linalg::Norm;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::{Normal, StandardNormal, Uniform};
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

    /// Evaluates the feasibility of the optimization problem
    /// $\min 0$ \
    /// $\text{subject to } A^\top x \leq b$ \
    fn empty(&self) -> Result<bool, SetOperationError> {
        // Define variables x_0, ..., x_{n-1} (unbounded)
        let mut vars = variables!();
        let x: Vec<_> = (0..self.dim()).map(|_| vars.add(variable())).collect();

        // Build the problem with dummy objective
        let mut problem = vars.minimise(0.0).using(default_solver);

        // Add constraints row by row: A[i] ⋅ x <= b[i]
        for i in 0..self.n_constraints() {
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

    /// Solves the optimization problem: \
    /// $\max c^\top x $ \
    /// $\text{subject to } A^\top x \leq b$ \
    fn center(&self) -> Result<Array1<f64>, SetOperationError> {
        let mut vars = variables!();
        let r = vars.add(variable().min(0.0));
        let x: Vec<_> = (0..self.dim()).map(|_| vars.add(variable())).collect();

        // maximize radius
        let mut problem = vars.maximise(r).using(default_solver);

        // constraints: a_i^T x + ||a_i|| * r <= b_i
        for (i, row) in self.A.outer_iter().enumerate() {
            let norm_ai = row.dot(&row).sqrt();
            let lhs: Expression = row.iter().zip(&x).map(|(&aij, &xj)| aij * xj).sum();
            problem = problem.with((lhs + norm_ai * r).leq(self.b[i]));
        }

        let solution = problem
            .solve()
            .map_err(|e| SetOperationError::InfeasibleOptimization {
                source: Box::new(e),
            })?;

        let center =
            Array1::from_shape_vec(self.dim(), x.iter().map(|&xi| solution.value(xi)).collect())
                .map_err(|e| SetOperationError::DataConversionError {
                    source: Box::new(e),
                })?;

        Ok(center)
    }

    /// Solves the optimization problem: \
    /// $\max d^\top x $ \
    /// $\text{subject to } A^\top x \leq b$ \
    fn support_function(
        &self,
        direction: Array1<f64>,
    ) -> Result<(Array1<f64>, f64), SetOperationError> {
        self._check_operand_dim(direction.dim())?;

        // Define variables for the support vector x_0, ..., x_{n-1} (unbounded)
        let mut vars = variables!();
        let x: Vec<_> = (0..self.dim()).map(|_| vars.add(variable())).collect();

        // Dot product objective
        let objective: Expression = direction.iter().zip(&x).map(|(d_i, x_i)| *d_i * *x_i).sum();
        let mut problem = vars.maximise(objective.clone()).using(default_solver);

        // Add constraints row by row: A[i] ⋅ x <= b[i]
        for i in 0..self.n_constraints() {
            let row = self.A.row(i);
            let expr: Expression = row.iter().zip(&x).map(|(coef, xi)| *coef * *xi).sum();
            problem = problem.with(expr.leq(self.b[i]));
        }

        let solution = problem
            .solve()
            .map_err(|e| SetOperationError::InfeasibleOptimization {
                source: Box::new(e),
            })?;

        let support_vector =
            Array1::from_shape_vec(self.dim(), x.iter().map(|&xi| solution.value(xi)).collect())
                .map_err(|e| SetOperationError::DataConversionError {
                    source: Box::new(e),
                })?;

        let support_value = solution.eval(&objective);

        Ok((support_vector, support_value))
    }

    fn volume(&self) -> Result<f64, SetOperationError> {
        let vpoly = VPolytope::new(self.to_vertices()?).map_err(|_| SetOperationError::EmptySet)?;
        vpoly.volume()
    }

    fn minkowski_sum_(&mut self, other: &Self) -> Result<(), SetOperationError> {
        // Implementation based on the support functions of both
        let dim = self.dim();

        // Collect candidate directions: normals from both polytopes
        let directions = ndarray::concatenate![Axis(0), self.A.view(), other.A.view()];

        // Prepare storage for new A and b
        let n_dirs = directions.nrows();
        let mut new_A = Array2::<f64>::zeros((n_dirs, dim));
        let mut new_b = Array1::<f64>::zeros(n_dirs);

        for (i, dir) in directions.outer_iter().enumerate() {
            // Normalize direction to avoid scaling issues
            let norm = dir.norm_l2();
            if norm < 1e-9 {
                continue;
            }
            let u = &dir / norm;

            // Compute support values
            let (_, h1) = self.support_function(u.clone())?;
            let (_, h2) = other.support_function(u.clone())?;

            // Fill row in A and entry in b
            new_A.row_mut(i).assign(&u);
            new_b[i] = h1 + h2;
        }

        // Replace self with new H-representation
        self.A = new_A;
        self.b = new_b;

        Ok(())
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

    fn contains_point(&self, point: Array1<f64>) -> Result<bool, SetOperationError> {
        self._check_operand_dim(point.dim())?;
        Ok(vector_leq(&self.A.dot(&point), &self.b))
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
