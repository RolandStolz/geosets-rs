use geosets_rs::sets::hpolytope::HPolytope;
use ndarray::Array;

#[test]
fn test_polytope_new() {
    let _ = HPolytope::new(Array::ones((2, 2)), Array::ones(2)).unwrap();
    let _ = HPolytope::new(Array::ones((5, 2)), Array::ones(5)).unwrap();
}
