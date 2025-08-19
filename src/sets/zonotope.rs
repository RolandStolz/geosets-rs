#![allow(unused)]
use super::*;
use crate::geometric_operations::convex_hull;
use crate::linalg_utils::rank;
use ndarray::Shape;
use ndarray_rand::rand_distr::{Exp1, Uniform};
use ndarray_rand::RandomExt;
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
        let hull_vertices = convex_hull(vertices)?;
        Ok(hull_vertices)
    }

    fn center(&self) -> Result<Array1<f64>, SetOperationError> {
        Ok(self.c.clone())
    }

    fn support_function(&self) -> Result<(Array1<f64>, f64), SetOperationError> {
        todo!()
    }

    fn volume(&self) -> Result<f64, SetOperationError> {
        todo!()
    }

    fn minkowski_sum_(&mut self, other: &Self) -> Result<(), SetOperationError> {
        self._check_operand_dim(other.dim())?;
        self.G = concatenate![Axis(1), self.G.clone(), other.G.clone()];
        // self.G = ndarray::concatenate(Axis(1), &[self.G.view(), other.G.view()]);
        self.c = &self.c + &other.c;
        Ok(())
    }

    fn matmul_(&mut self, mat: &Array2<f64>) -> Result<(), SetOperationError> {
        self._check_operand_dim(mat.dim().0)?;
        self.c = mat.dot(&self.c);
        self.G = mat.dot(&self.G);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zonotope_new() {
        let _ = Zonotope::new(Array::ones((5, 2)), Array::zeros(2)).unwrap();
        let zono = Zonotope::new(Array::eye(3), Array::zeros(2));

        // Expect an error when unwrapping zono2
        assert!(zono.is_err());
    }
}
