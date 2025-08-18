#![allow(unused)]
use super::*;
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
        if G.dim().0 != c.dim() {
            Err(ZonotopeError::DimensionMismatch {
                g_dim: G.dim(),
                c_dim: c.dim(),
            })
        } else {
            Ok(Zonotope { G, c })
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zonotope_new() {
        let _ = Zonotope::new(Array::ones((2, 5)), Array::zeros(2)).unwrap();
        let zono = Zonotope::new(Array::eye(3), Array::zeros(2));

        // Expect an error when unwrapping zono2
        assert!(zono.is_err());
    }

    #[test]
    fn test_minkowski_sum() {
        let zono1 = Zonotope::new(Array::ones((2, 3)), Array::zeros(2)).unwrap();
        let zono2 = Zonotope::new(Array::zeros((2, 3)), Array::ones(2)).unwrap();

        let zono3 = zono1.minkowski_sum(&zono2).unwrap();

        assert_eq!(
            zono3.G,
            array![[1., 1.], [1., 1.], [1., 1.], [0., 0.], [0., 0.], [0., 0.],].t(),
        );
        assert_eq!(zono3.c, array![1., 1.]);

        // Dimension test
        let zono1 = Zonotope::new(Array::ones((3, 3)), Array::zeros(3)).unwrap();
        let zono2 = Zonotope::new(Array::zeros((2, 3)), Array::ones(2)).unwrap();

        assert!(zono1.minkowski_sum(&zono2).is_err());
    }

    #[test]
    fn test_linear_transform() {
        let zono1 = Zonotope::new(Array::ones((2, 3)), Array::zeros(2)).unwrap();
        let mat = Array::eye(2);

        let zono2 = zono1.matmul(&mat).unwrap();

        assert_eq!(zono2.G, array![[1., 1.], [1., 1.], [1., 1.]].t());
        assert_eq!(zono2.c, array![0., 0.]);

        // Dimension check
        let zono1 = Zonotope::new(Array::ones((2, 3)), Array::zeros(2)).unwrap();
        let mat = Array::eye(3);

        assert!(zono1.matmul(&mat).is_err());
    }
}
