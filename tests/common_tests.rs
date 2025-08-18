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
        #[case::zonotope(std::marker::PhantomData::<geosets_rs::VPolytope>)]
        #[case::zonotope(std::marker::PhantomData::<geosets_rs::Zonotope>)]
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

test_all_geosets!(test_translate_common, {
    for dim in 2..5 {
        let set = T::from_unit_box(dim);
        // Create a translation vector filled with 1.0
        let translation = Array1::from_elem(dim, 1.0);

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
