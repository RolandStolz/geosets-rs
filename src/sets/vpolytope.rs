#![allow(unused)]
use super::*;
use plotly::common::Mode;
use plotly::{Plot, Scatter};

#[derive(Clone, Debug)]
pub struct VPolytope {
    vertices: Array2<f64>,
}

impl VPolytope {
    pub fn new(vertices: Array2<f64>) -> Result<VPolytope, SetOperationError> {
        Ok(VPolytope { vertices })
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
        Ok(self.vertices.clone())
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

    fn minkowski_sum(&self, other: &Self) -> Result<Self, SetOperationError> {
        todo!()
    }

    fn minkowski_sum_(&self, other: &Self) -> Result<(), SetOperationError> {
        todo!()
    }

    fn matmul(&self, mat: &Array2<f64>) -> Result<Self, SetOperationError> {
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
        let _ = VPolytope::new(Array::ones((2, 2))).unwrap();
        let _ = VPolytope::new(Array::ones((2, 5))).unwrap();
    }
}
