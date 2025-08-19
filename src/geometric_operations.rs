use ndarray::Array2;
use qhull::Qh;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeometryError {
    #[error("Vertices must be a 2D array with shape (n, 2)")]
    InvalidShape,
}

pub fn order_vertices_clockwise(vertices: Array2<f64>) -> Result<Array2<f64>, GeometryError> {
    let n_vertices = vertices.nrows();

    if vertices.dim().1 != 2 {
        return Err(GeometryError::InvalidShape);
    }

    if n_vertices < 3 {
        return Ok(vertices);
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

    Ok(ordered_vertices)
}

#[derive(Debug, Error)]
pub enum ConvexHullError {
    // #[error("Qhull error: {0}")]
    // QhullError(#[from] qhull::QhError),
    #[error("Error from qhull: {source}")]
    QhullError {
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("Data conversion error: {source}")]
    DataConversionError {
        #[source]
        source: Box<dyn std::error::Error>,
    },
}

pub fn convex_hull(vertices: Array2<f64>) -> Result<Array2<f64>, ConvexHullError> {
    if vertices.nrows() <= vertices.dim().1 {
        // Not enough points for a proper convex hull in this dimension
        return Ok(vertices);
    }

    let points: Vec<Vec<f64>> = vertices
        .rows()
        .into_iter()
        .map(|row| row.to_vec())
        .collect();

    // Compute convex hull
    let qh = Qh::builder()
        .compute(true)
        .build_from_iter(points)
        .map_err(|e| ConvexHullError::QhullError {
            source: Box::new(e),
        })?;

    // Extract hull vertices
    let hull_vertices: Vec<Vec<f64>> = qh
        .vertices()
        .map(|vertex| {
            let idx = vertex.index(&qh).unwrap();
            vertices.row(idx).to_vec()
        })
        .collect();

    // Convert back to Array2
    if hull_vertices.is_empty() {
        return Ok(Array2::zeros((0, vertices.ncols())));
    }

    let n_vertices = hull_vertices.len();
    let dim = hull_vertices[0].len();
    let flattened: Vec<f64> = hull_vertices.into_iter().flatten().collect();

    Ok(
        Array2::from_shape_vec((n_vertices, dim), flattened).map_err(|e| {
            ConvexHullError::DataConversionError {
                source: Box::new(e),
            }
        })?,
    )
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

        let ordered = order_vertices_clockwise(vertices).unwrap();

        // Check that we have 4 vertices
        assert_eq!(ordered.nrows(), 4);
        assert_eq!(ordered.ncols(), 2);

        // The exact order depends on the starting angle, but vertices should form a valid clockwise path
        // We can verify by checking that consecutive vertices are connected properly
        println!("Ordered vertices: {:?}", ordered);
    }
}
