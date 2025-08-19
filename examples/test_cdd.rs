use geosets_rs::cddlib_rs::compute_polytope_vertices;
use ndarray::{Array1, Array2};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing cddlib integration...");

    // Simple 2D unit box: x >= 0, y >= 0, x <= 1, y <= 1
    // A * x <= b
    // [[1, 0], [0, 1], [-1, 0], [0, -1]] * [x, y] <= [1, 1, 0, 0]
    let a = Array2::from_shape_vec(
        (4, 2),
        vec![
            1.0, 0.0, // x <= 1
            0.0, 1.0, // y <= 1
            -1.0, 0.0, // -x <= 0 => x >= 0
            0.0, -1.0, // -y <= 0 => y >= 0
        ],
    )?;
    let b = Array1::from_vec(vec![1.0, 1.0, 0.0, 0.0]);

    println!("A matrix: {:?}", a);
    println!("b vector: {:?}", b);

    let vertices = compute_polytope_vertices(&a, &b)?;
    println!("Vertices: {:?}", vertices);

    Ok(())
}
