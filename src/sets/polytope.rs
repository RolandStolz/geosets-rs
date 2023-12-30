use super::*;

struct Polytope {
    vertices: Array2<f64>,
}

impl Polytope {
    fn new(vertices: Array2<f64>) -> Result<Polytope, PolytopeError> {
        Ok(Polytope { vertices })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polytope_new() {
        let _ = Polytope::new(Array::ones((2, 2))).unwrap();
        let _ = Polytope::new(Array::ones((2, 5))).unwrap();
    }
}
