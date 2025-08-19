use geosets_rs::order_vertices_clockwise;
use geosets_rs::sets::GeoSet;
use ndarray::{array, Array1, Array2};
use rstest::rstest;
use std::collections::HashSet;

// This module contains common tests for the GeoSet trait.
// These tests are implementation-agnostic and apply to any type implementing GeoSet.

// Macro to generate test functions for all GeoSet implementations
macro_rules! test_all_geosets {
    ($test_name:ident, $test_body:block) => {
        #[rstest]
        #[case::hpolytope(std::marker::PhantomData::<geosets_rs::HPolytope>)]
        #[case::vpolytope(std::marker::PhantomData::<geosets_rs::VPolytope>)]
        #[case::zonotope(std::marker::PhantomData::<geosets_rs::Zonotope>)]
        #[case::interval(std::marker::PhantomData::<geosets_rs::Interval>)]
        fn $test_name<T: GeoSet>(#[case] _marker: std::marker::PhantomData<T>) $test_body
    };
}

// Used for testing
fn to_set(vertices: &Array2<f64>) -> HashSet<Vec<u64>> {
    vertices
        .outer_iter()
        .map(|row| row.iter().map(|&x| x.to_bits()).collect::<Vec<u64>>())
        .collect::<HashSet<_>>()
}

// Generate test functions using the macro
test_all_geosets!(test_dim_common, {
    for dim in 2..5 {
        let set = T::from_unit_box(dim);
        assert_eq!(
            set.dim(),
            dim,
            "Dimension mismatch for unit box of dimension {}",
            dim
        );
    }
});

test_all_geosets!(test_empty_common, {
    let set = T::from_unit_box(2);
    assert!(!set.empty().unwrap(), "Unit box should not be empty");
});

test_all_geosets!(test_center_common, {
    for dim in 2..5 {
        let set = T::from_unit_box(dim);
        if let Ok(center) = set.center() {
            assert_eq!(
                center.len(),
                dim,
                "Center dimension should match the set dimension"
            );

            // For a unit box centered at the origin, all coordinates should be 0
            for val in center.iter() {
                assert_eq!(*val, 0.0, "Center of unit box should be at the origin");
            }
        }
    }
});

test_all_geosets!(test_translate_common, {
    for dim in 2..5 {
        let set = T::from_unit_box(dim);
        // Create a translation vector filled with 1.0
        let translation = Array1::ones(dim);

        if let Ok(translated) = set.translate(&translation) {
            // The dimension should remain the same
            assert_eq!(
                translated.dim(),
                dim,
                "Dimension should not change after translation"
            );

            // The center should be translated by the vector
            if let (Ok(orig_center), Ok(new_center)) = (set.center(), translated.center()) {
                for i in 0..dim {
                    assert_eq!(
                        new_center[i],
                        orig_center[i] + translation[i],
                        "Center should be translated by the vector"
                    );
                }
            }
        }
    }
});

test_all_geosets!(test_to_vertices_common, {
    // 2d
    let set = T::from_unit_box(2);
    let vertices = set.to_vertices().unwrap();

    assert_eq!(vertices.dim().0, 4, "2D unit box should have 4 vertices");
    assert_eq!(vertices.dim().1, 2, "2D vertices should have 2 coordinates");
    let ordered_vertices = order_vertices_clockwise(vertices).unwrap();

    assert_eq!(ordered_vertices.row(0), array![-1.0, 1.0]);
    assert_eq!(ordered_vertices.row(1), array![1.0, 1.0]);
    assert_eq!(ordered_vertices.row(2), array![1.0, -1.0]);
    assert_eq!(ordered_vertices.row(3), array![-1.0, -1.0]);

    // 3d
    let set = T::from_unit_box(3);
    let vertices = set.to_vertices().unwrap();
    assert_eq!(vertices.dim().0, 8, "3D unit box should have 8 vertices");
    assert_eq!(vertices.dim().1, 3, "3D vertices should have 3 coordinates");

    let expected = array![
        [1.0, -1.0, -1.0],
        [1.0, -1.0, 1.0],
        [1.0, 1.0, 1.0],
        [1.0, 1.0, -1.0],
        [-1.0, 1.0, 1.0],
        [-1.0, 1.0, -1.0],
        [-1.0, -1.0, 1.0],
        [-1.0, -1.0, -1.0],
    ];

    assert_eq!(to_set(&vertices), to_set(&expected));
});
