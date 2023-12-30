use super::*;

#[allow(non_snake_case)]
struct Zonotope {
    G: Array2<f64>,
    c: Array1<f64>,
}

#[allow(non_snake_case)]
impl Zonotope {
    fn new(G: Array2<f64>, c: Array1<f64>) -> Result<Zonotope, ZonotopeError> {
        if G.dim().0 != c.dim() {
            Err(ZonotopeError::DimensionMismatch)
        } else {
            Ok(Zonotope { G, c })
        }
    }
}

impl GeoSet for Zonotope {
    fn dim(&self) -> usize {
        self.c.dim()
    }

    fn to_vertices(&self) -> Array2<f64> {
        Array::ones((self.dim(), 5))
    }

    fn minkowski_sum(&self, other: &Self) -> Result<Self, SetOperationError> {
        if self.dim() != other.dim() {
            Err(SetOperationError::DimensionMismatch)
        } else {
            Ok(Self::new(
                concatenate![Axis(1), self.G, other.G],
                self.c.clone() + other.c.clone(),
            )
            .unwrap())
        }
    }

    fn linear_transform(&self, mat: &Array2<f64>) -> Result<Self, SetOperationError> {
        if self.dim() != mat.dim().0 {
            Err(SetOperationError::DimensionMismatch)
        } else {
            Ok(Self::new(mat.dot(&self.G), mat.dot(&self.c)).unwrap())
        }
    }

    fn scalar_product(&self, scalar: f64) -> Result<Self, SetOperationError> {
        Ok(Self::new(scalar * self.G.clone(), scalar * self.c.clone()).unwrap())
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

        let zono2 = zono1.linear_transform(&mat).unwrap();

        assert_eq!(zono2.G, array![[1., 1.], [1., 1.], [1., 1.]].t());
        assert_eq!(zono2.c, array![0., 0.]);

        // Dimension check
        let zono1 = Zonotope::new(Array::ones((2, 3)), Array::zeros(2)).unwrap();
        let mat = Array::eye(3);

        assert!(zono1.linear_transform(&mat).is_err());
    }

}
