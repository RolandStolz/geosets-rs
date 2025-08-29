#![allow(unused)]
use super::*;
use crate::linalg_utils::{rank, sign, vector_all_close};
use crate::qhull_wrapper::convex_hull_vertices;
use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variable, variables};
use itertools::Itertools;
use ndarray::Shape;
use ndarray_linalg::Determinant;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::{Exp1, Uniform};
use qhull::Qh;
use thiserror::Error;

#[derive(Clone, Debug)]
#[allow(non_snake_case)]
pub struct Zonotope {
    G: Array2<f64>,
    c: Array1<f64>,
}

#[derive(Error, Debug)]
pub enum ZonotopeError {
    #[error("Dimensions of G {g_dim:?} and c {c_dim:?} do not match")]
    DimensionMismatch { g_dim: (usize, usize), c_dim: usize },
}

#[allow(non_snake_case)]
impl Zonotope {
    pub fn new(G: Array2<f64>, c: Array1<f64>) -> Result<Zonotope, ZonotopeError> {
        if G.dim().1 != c.dim() {
            Err(ZonotopeError::DimensionMismatch {
                g_dim: G.dim(),
                c_dim: c.dim(),
            })
        } else {
            Ok(Zonotope { G, c })
        }
    }

    pub fn from_random(
        dim: usize,
        n_generators: usize,
        zero_centered: bool,
    ) -> Result<Zonotope, ZonotopeError> {
        let mut G = Array2::random((n_generators, dim), Exp1);

        let mut c = if zero_centered {
            Array1::zeros(dim)
        } else {
            Array1::random(dim, Uniform::new(-0.2, 0.2))
        };

        let max_deviation = c.abs() + G.abs().sum_axis(Axis(0));
        // get max of max_deviation as scalar
        let scale = max_deviation.fold(0.0_f64, |a, &b| a.max(b));

        if scale > 1. {
            G /= scale;
            c /= scale;
        }

        Ok(Zonotope { G, c })
    }

    pub fn n_generators(&self) -> usize {
        self.G.nrows()
    }

    pub fn is_zero_centered(&self) -> bool {
        self.c.iter().all(|&x| x.abs() < 1e-9)
    }

    pub fn zonotope_norm(&self, point: &Array1<f64>) -> Result<f64, SetOperationError> {
        self._check_operand_dim(point.dim())?;

        // if !self.is_zero_centered() {
        //     return Err(SetOperationError::UnsupportedOperation {
        //         message: "Zonotope must be zero-centered",
        //     });
        // }

        let m = self.n_generators();
        if self.degenerate() {
            if vector_all_close(point, &self.c, 1e-9) {
                return Ok(0.0);
            } else {
                return Ok(f64::INFINITY);
            }
        }

        // Optimization problem

        let mut vars = variables!();
        let lambda = vars.add(variable().min(0));
        let alpha: Vec<_> = (0..self.n_generators())
            .map(|_| vars.add(variable()))
            .collect();

        let objective: Expression = lambda.into();
        let mut problem = vars.minimise(objective).using(default_solver);

        // G \alpha = x
        for i in 0..self.dim() {
            let g = &self.G.column(i);
            // Iterate over dimensions, not generators
            let expr: Expression = g
                .iter()
                .zip(&alpha)
                .map(|(g_i, alpha_i)| *g_i * *alpha_i)
                .sum();
            problem = problem.with(expr.eq(point[i] - self.c[i]));
        }

        // \alpha \leq \lamdba, \alpha \geq -\lambda
        for alpha_i in &alpha {
            problem = problem.with(constraint!(*alpha_i <= lambda));
            problem = problem.with(constraint!(*alpha_i >= -lambda));
        }

        match problem.solve() {
            Ok(solution) => {
                let lambda_val = solution.value(lambda);
                Ok(lambda_val)
            }
            Err(_) => Ok(f64::INFINITY),
        }
    }
}

#[allow(non_snake_case)]
impl GeoSet for Zonotope {
    fn from_unit_box(dim: usize) -> Self {
        let G = Array2::eye(dim);
        let c = Array1::zeros(dim);
        Zonotope::new(G, c).unwrap()
    }

    fn dim(&self) -> usize {
        self.c.dim()
    }

    fn empty(&self) -> Result<bool, SetOperationError> {
        Ok(false)
    }

    fn to_vertices(&self) -> Result<Array2<f64>, SetOperationError> {
        let mut vertices = self.c.clone().into_shape_clone((1, self.dim())).unwrap();

        for i in 0..self.n_generators() {
            vertices = ndarray::concatenate(
                Axis(0),
                &[
                    (&vertices + &self.G.row(i)).view(),
                    (&vertices - &self.G.row(i)).view(),
                ],
            )
            .unwrap();
        }

        // Compute convex hull using qhull -> automatically propagates error
        let hull_vertices = convex_hull_vertices(&vertices)?;
        Ok(hull_vertices)
    }

    fn center(&self) -> Result<Array1<f64>, SetOperationError> {
        Ok(self.c.clone())
    }

    fn support_function(
        &self,
        direction: Array1<f64>,
    ) -> Result<(Array1<f64>, f64), SetOperationError> {
        self._check_operand_dim(direction.dim())?;

        let projection = self.G.dot(&direction);
        // signum is not correct here!
        let projection_sign = sign(&projection);

        let support_value = direction.dot(&self.c) + projection.abs().sum();
        let support_vector = &self.c + projection_sign.dot(&self.G);

        Ok((support_vector, support_value))
    }

    fn volume(&self) -> Result<f64, SetOperationError> {
        if self.degenerate() {
            return Ok(0.0);
        }

        let all_combinations = (0..self.n_generators()).combinations(self.dim());

        let mut vol = 0.0;
        for comb in all_combinations {
            let submatrix = self.G.select(Axis(0), &comb);
            vol += submatrix.det().unwrap().abs();
        }
        Ok(2.0_f64.powf(self.dim() as f64) * vol)
    }

    fn minkowski_sum_(&mut self, other: &Self) -> Result<(), SetOperationError> {
        self._check_operand_dim(other.dim())?;
        self.G = concatenate![Axis(0), self.G.clone(), other.G.clone()];
        // self.G = ndarray::concatenate(Axis(1), &[self.G.view(), other.G.view()]);
        self.c = &self.c + &other.c;
        Ok(())
    }

    fn matmul_(&mut self, mat: &Array2<f64>) -> Result<(), SetOperationError> {
        self._check_operand_dim(mat.dim().0)?;
        self.c = mat.dot(&self.c);
        self.G = self.G.dot(&mat.t());
        Ok(())
    }

    fn translate_(&mut self, vector: &Array1<f64>) -> Result<(), SetOperationError> {
        self._check_operand_dim(vector.dim())?;
        self.c = &self.c + vector;
        Ok(())
    }

    fn degenerate(&self) -> bool {
        self.n_generators() == 0 || rank(&self.G).unwrap() < self.dim()
    }

    fn contains_point(&self, point: &Array1<f64>) -> Result<bool, SetOperationError> {
        Ok(self.zonotope_norm(point)? <= 1.0 + 1e-9)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_zonotope_new() {
        let _ = Zonotope::new(Array::ones((5, 2)), Array::zeros(2)).unwrap();
        let zono = Zonotope::new(Array::eye(3), Array::zeros(2));

        // Expect an error when unwrapping zono2
        assert!(zono.is_err());
    }

    #[test]
    fn test_matmul_rotation() {
        let G = array![[1.0, 0.0], [0.0, 1.0]]; // Two generators: [1,0] and [0,1]
        let c = array![2.0, 3.0]; // Center at (2,3)
        let mut zono = Zonotope::new(G, c).unwrap();

        // Apply a 90-degree rotation matrix
        let rotation_90 = array![[0.0, -1.0], [1.0, 0.0]];

        zono.matmul_(&rotation_90).unwrap();

        // After rotation:
        // - Center (2,3) should become (-3,2)
        // - Generator [1,0] should become [0,1]
        // - Generator [0,1] should become [-1,0]

        let expected_c = array![-3.0, 2.0];
        let expected_G = array![[0.0, 1.0], [-1.0, 0.0]];

        assert!(zono.c.abs_diff_eq(&expected_c, 1e-10));
        assert!(zono.G.abs_diff_eq(&expected_G, 1e-10));
    }

    #[test]
    fn test_matmul_scaling() {
        let G = array![[2.0, 1.0], [1.0, 3.0]];
        let c = array![1.0, 2.0];
        let mut zono = Zonotope::new(G, c).unwrap();

        // Scale by 2 in x, 3 in y
        let scale = array![[2.0, 0.0], [0.0, 3.0]];

        zono.matmul_(&scale).unwrap();

        // Center should be scaled
        let expected_c = array![2.0, 6.0];
        // Generators should be transformed: [2,1] -> [4,3], [1,3] -> [2,9]
        let expected_G = array![[4.0, 3.0], [2.0, 9.0]];

        assert!(zono.c.abs_diff_eq(&expected_c, 1e-10));
        assert!(zono.G.abs_diff_eq(&expected_G, 1e-10));
    }
}
