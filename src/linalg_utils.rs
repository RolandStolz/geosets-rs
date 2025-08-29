use ndarray::{Array1, Array2};
use ndarray_linalg::SVD;

pub fn rank(array: &Array2<f64>) -> Result<usize, Box<dyn std::error::Error>> {
    let (_u, s, _vt) = array.svd(true, true)?;

    // Count non-zero singular values (with tolerance for numerical precision)
    let tolerance = 1e-10;
    let rank = s.iter().filter(|&&value| value.abs() > tolerance).count();

    Ok(rank)
}

pub fn argmax(vector: &Array1<f64>) -> Option<usize> {
    vector
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i)
}

pub fn sign(vector: &Array1<f64>) -> Array1<f64> {
    vector.mapv(|x| {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            0.0
        }
    })
}

pub fn vector_leq(a: &Array1<f64>, b: &Array1<f64>) -> bool {
    a.iter().zip(b.iter()).all(|(a_i, b_i)| a_i <= b_i)
}

pub fn vector_all_close(a: &Array1<f64>, b: &Array1<f64>, tol: f64) -> bool {
    a.iter()
        .zip(b.iter())
        .all(|(a_i, b_i)| (a_i - b_i).abs() <= tol)
}
