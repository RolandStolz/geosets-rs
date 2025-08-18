use geosets_rs::sets::GeoSet;
use rstest::rstest;

// This module contains common tests for the GeoSet trait.
// These tests are implementation-agnostic and apply to any type implementing GeoSet.

// Macro to generate test functions for all GeoSet implementations
macro_rules! test_all_geosets {
    ($test_name:ident, $test_body:block) => {
        #[rstest]
        #[case::hpolytope(std::marker::PhantomData::<geosets_rs::HPolytope>)]
        #[case::zonotope(std::marker::PhantomData::<geosets_rs::Zonotope>)]
        fn $test_name<T: GeoSet>(#[case] _marker: std::marker::PhantomData<T>) $test_body
    };
}

// Generate test functions using the macro
test_all_geosets!(test_dim_common, {
    for dim in 2..5 {
        if let Ok(set) = T::from_unit_box(dim) {
            assert_eq!(
                set.dim(),
                dim,
                "Dimension mismatch for unit box of dimension {}",
                dim
            );
        }
    }
});

test_all_geosets!(test_empty_common, {
    if let Ok(set) = T::from_unit_box(2) {
        assert!(!set.empty(), "Unit box should not be empty");
    }
});
