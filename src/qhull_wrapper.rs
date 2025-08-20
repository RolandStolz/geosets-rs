use ndarray::{Array1, Array2, Axis};
use ndarray_linalg::Determinant;
use qhull::Qh;
use thiserror::Error;

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

    #[error("Insufficient point")]
    InsufficientPoints,
}

pub fn convex_hull(vertices: &Array2<f64>, triangulate: bool) -> Result<Qh<'static>, ConvexHullError> {
    if vertices.nrows() <= vertices.dim().1 {
        // Not enough points for a proper convex hull in this dimension
        return Err(ConvexHullError::InsufficientPoints);
    }

    let points: Vec<Vec<f64>> = vertices
        .rows()
        .into_iter()
        .map(|row| row.to_vec())
        .collect();

    // Compute convex hull with triangulation enabled
    Qh::builder()
        .compute(true)
        .triangulate(triangulate)
        // .triangulate(true)  // Enable triangulation to get simplices
        .build_from_iter(points)
        .map_err(|e| ConvexHullError::QhullError {
            source: Box::new(e),
        })
}

pub fn convex_hull_vertices(vertices: &Array2<f64>) -> Result<Array2<f64>, ConvexHullError> {
    let qh = convex_hull(vertices, false)
        .map_err(|e| ConvexHullError::QhullError {
            source: Box::new(e),
        })
        .unwrap();

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

/// Compute the volume of an n-dimensional simplex
fn simplex_volume(vertices: &[Array1<f64>]) -> f64 {
    if vertices.len() <= 1 {
        return 0.0;
    }

    let n = vertices.len() - 1;
    if n == 0 {
        return 0.0;
    }

    // Create matrix of edge vectors from first vertex
    let mut matrix = Array2::<f64>::zeros((n, n));
    for i in 0..n {
        let diff = &vertices[i + 1] - &vertices[0];
        for j in 0..n.min(diff.len()) {
            matrix[[i, j]] = diff[j];
        }
    }

    let det = matrix.det().unwrap_or(0.0);
    let factorial = (1..=n).product::<usize>() as f64;
    det.abs() / factorial
}

pub fn qhull_volume(qh: &Qh, vertices: &Array2<f64>) -> Result<f64, ConvexHullError> {
    if vertices.is_empty() || vertices.ncols() == 0 {
        return Ok(0.0);
    }

    // Compute centroid of all vertices
    let centroid = vertices.mean_axis(Axis(0)).unwrap();
    let mut total_volume = 0.0;

    for (_simplex_idx, simplex) in qh.simplices().enumerate() {
        let vertex_set = simplex
            .vertices()
            .ok_or_else(|| ConvexHullError::DataConversionError {
                source: "Failed to get facet vertices".into(),
            })?;

        let simplex_vertices: Vec<Array1<f64>> = vertex_set
            .iter()
            .filter_map(|vertex| {
                vertex
                    .index(&qh)
                    .map(|idx| vertices.row(idx).to_owned())
            })
            .collect();

        // Create pyramid with centroid as apex and simplex as base
        let mut pyramid_vertices = vec![centroid.clone()];
        pyramid_vertices.extend(simplex_vertices);

        let simplex_contribution = simplex_volume(&pyramid_vertices);
        // println!("Simplex contribution: {}", simplex_contribution);
        total_volume += simplex_contribution;
    }

    Ok(total_volume)
}
