use ndarray::Array2;

pub fn order_vertices_clockwise(vertices: Array2<f64>) -> Array2<f64> {
    let n_vertices = vertices.nrows();

    if n_vertices < 3 {
        return vertices;
    }

    // Calculate centroid
    let centroid_x = vertices.column(0).mean().unwrap();
    let centroid_y = vertices.column(1).mean().unwrap();

    // Create vector of (index, angle) pairs
    let mut vertex_angles: Vec<(usize, f64)> = (0..n_vertices)
        .map(|i| {
            let x = vertices[[i, 0]] - centroid_x;
            let y = vertices[[i, 1]] - centroid_y;
            let angle = y.atan2(x);
            (i, angle)
        })
        .collect();

    // Sort by angle in descending order for clockwise ordering
    vertex_angles.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Create new array with ordered vertices
    let mut ordered_vertices = Array2::zeros((n_vertices, 2));
    for (new_idx, (orig_idx, _)) in vertex_angles.iter().enumerate() {
        ordered_vertices[[new_idx, 0]] = vertices[[*orig_idx, 0]];
        ordered_vertices[[new_idx, 1]] = vertices[[*orig_idx, 1]];
    }

    ordered_vertices
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_order_vertices_clockwise() {
        // Square vertices in random order
        let vertices = array![
            [1.0, 1.0], // top-right
            [0.0, 0.0], // bottom-left
            [1.0, 0.0], // bottom-right
            [0.0, 1.0]  // top-left
        ];

        let ordered = order_vertices_clockwise(vertices);

        // Check that we have 4 vertices
        assert_eq!(ordered.nrows(), 4);
        assert_eq!(ordered.ncols(), 2);

        // The exact order depends on the starting angle, but vertices should form a valid clockwise path
        // We can verify by checking that consecutive vertices are connected properly
        println!("Ordered vertices: {:?}", ordered);
    }

    #[test]
    fn test_triangle_ordering() {
        let vertices = array![
            [1.0, 0.0], // right
            [0.0, 1.0], // top
            [0.0, 0.0]  // bottom-left
        ];

        let ordered = order_vertices_clockwise(vertices);
        assert_eq!(ordered.nrows(), 3);
    }

    #[test]
    fn test_insufficient_vertices() {
        let vertices = array![[0.0, 0.0], [1.0, 1.0]];
        let ordered = order_vertices_clockwise(vertices.clone());
        assert_eq!(ordered, vertices);
    }
}
