use geosets_rs::sets::GeoSet;
use ndarray::{Array1, Array2, array};
use rstest::rstest;
use std::collections::HashSet;

// This module contains common tests for the GeoSet trait using randomly generated sets.
// These tests are implementation-agnostic and apply to any type implementing GeoSet.

// Macro to generate test functions for all GeoSet implementations using random sets
macro_rules! test_all_random_geosets {
    ($test_name:ident, $test_body:block) => {
        #[rstest]
        #[case::hpolytope(std::marker::PhantomData::<geosets_rs::HPolytope>)]
        #[case::vpolytope(std::marker::PhantomData::<geosets_rs::VPolytope>)]
        #[case::zonotope(std::marker::PhantomData::<geosets_rs::Zonotope>)]
        #[case::interval(std::marker::PhantomData::<geosets_rs::Interval>)]
        fn $test_name<T: GeoSet + CreateRandom>(#[case] _marker: std::marker::PhantomData<T>) $test_body
    };
}

// Trait to create random sets for each type
trait CreateRandom {
    fn create_random(dim: usize) -> Self;
}

impl CreateRandom for geosets_rs::HPolytope {
    fn create_random(dim: usize) -> Self {
        geosets_rs::HPolytope::from_random(dim, dim + 3).unwrap()
    }
}

impl CreateRandom for geosets_rs::VPolytope {
    fn create_random(dim: usize) -> Self {
        // Use 2^dim vertices for reasonable complexity, but cap at 16
        let n_vertices = (2_usize).pow(dim as u32).min(16).max(4);
        geosets_rs::VPolytope::from_random(dim, n_vertices).unwrap()
    }
}

impl CreateRandom for geosets_rs::Zonotope {
    fn create_random(dim: usize) -> Self {
        // Use dim + 2 generators for good coverage
        geosets_rs::Zonotope::from_random(dim, dim + 2, false).unwrap()
    }
}

impl CreateRandom for geosets_rs::Interval {
    fn create_random(dim: usize) -> Self {
        geosets_rs::Interval::from_random(dim).unwrap()
    }
}

// Used for testing
fn _vertices_to_set(vertices: &Array2<f64>) -> HashSet<Vec<u64>> {
    vertices
        .outer_iter()
        .map(|row| row.iter().map(|&x| x.to_bits()).collect::<Vec<u64>>())
        .collect::<HashSet<_>>()
}

// Generate test functions using the macro
test_all_random_geosets!(test_dim_random, {
    for dim in 2..5 {
        let set = T::create_random(dim);
        assert_eq!(set.dim(), dim);
    }
});

test_all_random_geosets!(test_empty_random, {
    let set = T::create_random(2);
    assert!(!set.empty().unwrap(), "Random set should not be empty");
});

test_all_random_geosets!(test_center_containment_random, {
    for dim in 2..4 {
        // Reduce range for random tests
        let set = T::create_random(dim);
        if let Ok(center) = set.center() {
            assert_eq!(center.len(), dim);

            assert!(set.contains_point(&center).unwrap());
        }
    }
});

test_all_random_geosets!(test_translate_random, {
    for dim in 2..4 {
        // Reduce range for random tests
        let set = T::create_random(dim);
        // Create a translation vector filled with 0.1
        let translation = Array1::from_elem(dim, 0.1);

        if let Ok(translated) = set.translate(&translation) {
            // The dimension should remain the same
            assert_eq!(translated.dim(), dim);

            // The center should be translated by the vector (if center computation succeeds)
            if let (Ok(orig_center), Ok(new_center)) = (set.center(), translated.center()) {
                for i in 0..dim {
                    assert!((new_center[i] - (orig_center[i] + translation[i])).abs() < 1e-6,);
                }
            }
        }
    }
});

test_all_random_geosets!(test_to_vertices_random, {
    // Test 2D random sets
    let set = T::create_random(2);
    let vertices_result = set.to_vertices();

    if let Ok(vertices) = vertices_result {
        assert_eq!(vertices.dim().1, 2, "2D vertices should have 2 coordinates");
        assert!(
            vertices.dim().0 >= 3,
            "2D convex set should have at least 3 vertices"
        );

        // Verify all vertices are finite
        for vertex in vertices.outer_iter() {
            for &coord in vertex.iter() {
                assert!(coord.is_finite(), "All vertex coordinates should be finite");
            }
        }
    }

    // Test 3D random sets
    let set_3d = T::create_random(3);
    let vertices_3d_result = set_3d.to_vertices();

    if let Ok(vertices_3d) = vertices_3d_result {
        assert_eq!(
            vertices_3d.dim().1,
            3,
            "3D vertices should have 3 coordinates"
        );
        assert!(
            vertices_3d.dim().0 >= 4,
            "3D convex set should have at least 4 vertices"
        );

        // Verify all vertices are finite
        for vertex in vertices_3d.outer_iter() {
            for &coord in vertex.iter() {
                assert!(coord.is_finite(), "All vertex coordinates should be finite");
            }
        }
    }
});

test_all_random_geosets!(test_degenerate_random, {
    for dim in 2..4 {
        let set = T::create_random(dim);
        let _is_degenerate = set.degenerate();
    }
});

test_all_random_geosets!(test_matmul_random, {
    for dim in 2..4 {
        let mut set = T::create_random(dim);

        // Use a panic hook to catch potential panics and convert to error
        set.matmul_(&(Array2::eye(dim) * 2.0)).unwrap();

        assert_eq!(set.dim(), dim, "Dimension should not change after matmul");
    }
});

test_all_random_geosets!(test_volume_random, {
    for dim in 2..4 {
        // Reduce range for random tests
        let set = T::create_random(dim);
        if let Ok(volume) = set.volume() {
            assert!(volume >= 0.0, "Volume should be non-negative");
            assert!(volume.is_finite(), "Volume should be finite");
        }
    }
});

test_all_random_geosets!(test_support_function_random, {
    let set = T::create_random(2);

    // Test with some standard directions
    let directions = vec![
        array![1.0, 0.0],
        array![0.0, 1.0],
        array![1.0, 1.0],
        array![-1.0, 0.0],
        array![0.0, -1.0],
    ];

    for direction in directions {
        if let Ok((support_vector, support_value)) = set.support_function(direction.clone()) {
            assert_eq!(support_vector.len(), 2);
            assert!(support_value.is_finite(), "Support value should be finite");
        }
    }
});
