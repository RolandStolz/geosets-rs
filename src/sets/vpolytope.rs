#![allow(unused)]
use super::*;

pub struct VPolytope {
    vertices: Array2<f64>,
}

impl VPolytope {
    pub fn new(vertices: Array2<f64>) -> Result<VPolytope, SetOperationError> {
        Ok(VPolytope { vertices })
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
