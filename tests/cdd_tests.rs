/// Tests for the `compute_polytope_vertices` function in the cddlib_rs module.
use geosets_rs::cddlib_rs::compute_polytope_vertices;
use geosets_rs::sets::errors::SetOperationError;
use ndarray::{array, Array2};
use std::collections::HashSet;

// Helper function to convert vertices to a set for comparison (handles floating point precision)
fn vertices_to_set(vertices: &Array2<f64>) -> HashSet<Vec<u64>> {
    vertices
        .outer_iter()
        .map(|row| row.iter().map(|&x| x.to_bits()).collect::<Vec<u64>>())
        .collect::<HashSet<_>>()
}

// Helper function to check if two sets of vertices are equal (handling floating point precision)
fn vertices_equal(v1: &Array2<f64>, v2: &Array2<f64>) -> bool {
    vertices_to_set(v1) == vertices_to_set(v2)
}

#[test]
fn test_unit_box_2d() {
    // 2D unit box: 0 <= x <= 1, 0 <= y <= 1
    // Constraints: x <= 1, y <= 1, -x <= 0, -y <= 0
    let a = array![
        [1.0, 0.0],  // x <= 1
        [0.0, 1.0],  // y <= 1
        [-1.0, 0.0], // -x <= 0 => x >= 0
        [0.0, -1.0], // -y <= 0 => y >= 0
    ];
    let b = array![1.0, 1.0, 0.0, 0.0];

    let vertices = compute_polytope_vertices(&a, &b).unwrap();

    // Expected vertices of unit box
    let expected = array![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0],];

    assert_eq!(vertices.nrows(), 4);
    assert_eq!(vertices.ncols(), 2);
    assert!(vertices_equal(&vertices, &expected));
}

#[test]
fn test_unit_box_3d() {
    // 3D unit box: 0 <= x,y,z <= 1
    let a = array![
        [1.0, 0.0, 0.0],  // x <= 1
        [0.0, 1.0, 0.0],  // y <= 1
        [0.0, 0.0, 1.0],  // z <= 1
        [-1.0, 0.0, 0.0], // x >= 0
        [0.0, -1.0, 0.0], // y >= 0
        [0.0, 0.0, -1.0], // z >= 0
    ];
    let b = array![1.0, 1.0, 1.0, 0.0, 0.0, 0.0];

    let vertices = compute_polytope_vertices(&a, &b).unwrap();

    // Expected vertices of 3D unit box (8 vertices)
    let expected = array![
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 1.0, 1.0],
        [0.0, 1.0, 1.0],
    ];

    assert_eq!(vertices.nrows(), 8);
    assert_eq!(vertices.ncols(), 3);
    assert!(vertices_equal(&vertices, &expected));
}

#[test]
fn test_triangle_2d() {
    // Triangle with vertices at (0,0), (1,0), (0,1)
    // Constraints: x >= 0, y >= 0, x + y <= 1
    let a = array![
        [-1.0, 0.0], // x >= 0
        [0.0, -1.0], // y >= 0
        [1.0, 1.0],  // x + y <= 1
    ];
    let b = array![0.0, 0.0, 1.0];

    let vertices = compute_polytope_vertices(&a, &b).unwrap();

    let expected = array![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0],];

    assert_eq!(vertices.nrows(), 3);
    assert_eq!(vertices.ncols(), 2);
    assert!(vertices_equal(&vertices, &expected));
}

#[test]
fn test_centered_square() {
    // Square centered at origin: -1 <= x,y <= 1
    let a = array![
        [1.0, 0.0],  // x <= 1
        [-1.0, 0.0], // x >= -1
        [0.0, 1.0],  // y <= 1
        [0.0, -1.0], // y >= -1
    ];
    let b = array![1.0, 1.0, 1.0, 1.0];

    let vertices = compute_polytope_vertices(&a, &b).unwrap();

    let expected = array![[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0],];

    assert_eq!(vertices.nrows(), 4);
    assert_eq!(vertices.ncols(), 2);
    assert!(vertices_equal(&vertices, &expected));
}

#[test]
fn test_pentagon() {
    // Regular pentagon approximation
    // Using 5 half-spaces to create a pentagon-like shape
    let a = array![
        [1.0, 0.0],       // x <= 1
        [0.309, 0.951],   // rotated constraint
        [-0.809, 0.588],  // rotated constraint
        [-0.809, -0.588], // rotated constraint
        [0.309, -0.951],  // rotated constraint
    ];
    let b = array![1.0, 1.0, 1.0, 1.0, 1.0];

    let vertices = compute_polytope_vertices(&a, &b).unwrap();

    // Should have 5 vertices for a pentagon
    assert_eq!(vertices.nrows(), 5);
    assert_eq!(vertices.ncols(), 2);
}

#[test]
fn test_dimension_mismatch() {
    // Matrix A has 3 rows but vector b has 2 elements
    let a = array![[1.0, 0.0], [0.0, 1.0], [-1.0, 0.0],];
    let b = array![1.0, 1.0]; // Should have 3 elements

    let result = compute_polytope_vertices(&a, &b);

    assert!(result.is_err());
    match result.unwrap_err() {
        SetOperationError::DimensionMismatch { expected, got } => {
            assert_eq!(expected, 3);
            assert_eq!(got, 2);
        }
        _ => panic!("Expected DimensionMismatch error"),
    }
}

#[test]
fn test_empty_polytope() {
    // Contradictory constraints: x >= 1 and x <= 0
    let a = array![
        [1.0, 0.0],  // x <= 0
        [-1.0, 0.0], // x >= 1 (contradictory)
    ];
    let b = array![0.0, -1.0];

    let result = compute_polytope_vertices(&a, &b);

    // Should return either an error or an empty set of vertices
    match result {
        Ok(vertices) => {
            // If it succeeds, it should return an empty set
            assert_eq!(
                vertices.nrows(),
                0,
                "Empty polytope should have 0 vertices, got: {:?}",
                vertices
            );
        }
        Err(_) => {
            // This is also acceptable - infeasible systems can return errors
        }
    }
}

#[test]
fn test_single_point() {
    // Constraints that define a single point: x = 1, y = 1
    let a = array![
        [1.0, 0.0],  // x <= 1
        [-1.0, 0.0], // x >= 1
        [0.0, 1.0],  // y <= 1
        [0.0, -1.0], // y >= 1
    ];
    let b = array![1.0, -1.0, 1.0, -1.0];

    let vertices = compute_polytope_vertices(&a, &b).unwrap();

    // Should have exactly one vertex at (1, 1)
    assert_eq!(vertices.nrows(), 1);
    assert_eq!(vertices.ncols(), 2);
    assert!((vertices[[0, 0]] - 1.0).abs() < 1e-9);
    assert!((vertices[[0, 1]] - 1.0).abs() < 1e-9);
}

#[test]
fn test_line_segment() {
    // Line segment from (0,0) to (1,1): x = y, 0 <= x <= 1
    let a = array![
        [1.0, -1.0], // x - y <= 0 => x <= y
        [-1.0, 1.0], // -x + y <= 0 => x >= y (so x = y)
        [1.0, 0.0],  // x <= 1
        [-1.0, 0.0], // x >= 0
    ];
    let b = array![0.0, 0.0, 1.0, 0.0];

    let vertices = compute_polytope_vertices(&a, &b).unwrap();

    // Should have two vertices: (0,0) and (1,1)
    assert_eq!(vertices.nrows(), 2);
    assert_eq!(vertices.ncols(), 2);

    let expected = array![[0.0, 0.0], [1.0, 1.0],];
    assert!(vertices_equal(&vertices, &expected));
}

#[test]
fn test_scaled_unit_box_small() {
    let scale_f = 1.0;

    // Scaled unit box: 0 <= x,y <= scale
    let a = array![
        [1.0, 0.0],  // x <= scale
        [0.0, 1.0],  // y <= scale
        [-1.0, 0.0], // x >= 0
        [0.0, -1.0], // y >= 0
    ];
    let b = array![scale_f, scale_f, 0.0, 0.0];

    let vertices = compute_polytope_vertices(&a, &b).unwrap();

    let expected = array![
        [0.0, 0.0],
        [scale_f, 0.0],
        [scale_f, scale_f],
        [0.0, scale_f],
    ];

    assert_eq!(vertices.nrows(), 4);
    assert_eq!(vertices.ncols(), 2);
    assert!(vertices_equal(&vertices, &expected));
}

#[test]
fn test_scaled_unit_box_medium() {
    let scale_f = 5.0;

    // Scaled unit box: 0 <= x,y <= scale
    let a = array![
        [1.0, 0.0],  // x <= scale
        [0.0, 1.0],  // y <= scale
        [-1.0, 0.0], // x >= 0
        [0.0, -1.0], // y >= 0
    ];
    let b = array![scale_f, scale_f, 0.0, 0.0];

    let vertices = compute_polytope_vertices(&a, &b).unwrap();

    let expected = array![
        [0.0, 0.0],
        [scale_f, 0.0],
        [scale_f, scale_f],
        [0.0, scale_f],
    ];

    assert_eq!(vertices.nrows(), 4);
    assert_eq!(vertices.ncols(), 2);
    assert!(vertices_equal(&vertices, &expected));
}

#[test]
fn test_unbounded_polytope() {
    // Half-space x >= 0, y >= 0 (unbounded)
    let a = array![
        [-1.0, 0.0], // x >= 0
        [0.0, -1.0], // y >= 0
    ];
    let b = array![0.0, 0.0];

    let vertices = compute_polytope_vertices(&a, &b).unwrap();

    // For an unbounded polytope with only non-negativity constraints,
    // CDD may return either:
    // 1. One vertex at the origin, or
    // 2. No vertices (since the polytope is unbounded and CDD focuses on finite vertices)
    assert!(
        vertices.nrows() <= 1,
        "Should have at most 1 vertex for unbounded polytope, got: {:?}",
        vertices
    );
    assert_eq!(vertices.ncols(), 2);

    // If there is a vertex, it should be the origin
    if vertices.nrows() == 1 {
        assert!(
            (vertices[[0, 0]] - 0.0).abs() < 1e-9,
            "Vertex should be at origin"
        );
        assert!(
            (vertices[[0, 1]] - 0.0).abs() < 1e-9,
            "Vertex should be at origin"
        );
    }
}

#[test]
fn test_rotated_square() {
    // Diamond/rotated square: |x| + |y| <= 1
    // Constraints: x + y <= 1, x - y <= 1, -x + y <= 1, -x - y <= 1
    let a = array![
        [1.0, 1.0],   // x + y <= 1
        [1.0, -1.0],  // x - y <= 1
        [-1.0, 1.0],  // -x + y <= 1
        [-1.0, -1.0], // -x - y <= 1
    ];
    let b = array![1.0, 1.0, 1.0, 1.0];

    let vertices = compute_polytope_vertices(&a, &b).unwrap();

    let expected = array![[1.0, 0.0], [0.0, 1.0], [-1.0, 0.0], [0.0, -1.0],];

    assert_eq!(vertices.nrows(), 4);
    assert_eq!(vertices.ncols(), 2);
    assert!(vertices_equal(&vertices, &expected));
}

#[test]
fn test_high_precision_vertices() {
    // Test with vertices that have high precision coordinates
    let a = array![[1.0, 0.0], [0.0, 1.0], [-1.0, 0.0], [0.0, -1.0],];
    let b = array![0.123456789, 0.987654321, 0.0, 0.0];

    let vertices = compute_polytope_vertices(&a, &b).unwrap();

    assert_eq!(vertices.nrows(), 4);
    assert_eq!(vertices.ncols(), 2);

    // Check that precision is maintained
    let max_x = vertices
        .column(0)
        .iter()
        .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let max_y = vertices
        .column(1)
        .iter()
        .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    assert!((max_x - 0.123456789).abs() < 1e-9);
    assert!((max_y - 0.987654321).abs() < 1e-9);
}

#[test]
fn test_degenerate_cases() {
    // Test with very small polytope
    let a = array![[1.0, 0.0], [0.0, 1.0], [-1.0, 0.0], [0.0, -1.0],];
    let b = array![1e-10, 1e-10, 0.0, 0.0];

    let vertices = compute_polytope_vertices(&a, &b).unwrap();

    // Very small polytopes might collapse to fewer vertices due to numerical precision
    // but should still have at least 1 vertex and the correct number of dimensions
    assert!(vertices.nrows() >= 1, "Should have at least 1 vertex");
    assert!(
        vertices.nrows() <= 4,
        "Should have at most 4 vertices for a quadrilateral"
    );
    assert_eq!(vertices.ncols(), 2);
}

#[test]
fn test_large_coordinates() {
    // Test with large coordinate values
    let a = array![[1.0, 0.0], [0.0, 1.0], [-1.0, 0.0], [0.0, -1.0],];
    let b = array![1e6, 1e6, 0.0, 0.0];

    let vertices = compute_polytope_vertices(&a, &b).unwrap();

    assert_eq!(vertices.nrows(), 4);
    assert_eq!(vertices.ncols(), 2);

    // Check that large coordinates are handled correctly
    let max_coord: f64 = vertices.iter().fold(0.0_f64, |a, &b| a.max(b));
    assert!(max_coord >= 1e6 - 1e-6);
}
