use geosets_rs::sets::GeoSet;
use ndarray::Array1;
use rstest::rstest;

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
