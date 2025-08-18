use ndarray::Array2;
use ndarray_linalg::SVD;

pub fn rank(array: &Array2<f64>) -> Result<usize, Box<dyn std::error::Error>> {
    let (_u, s, _vt) = array.svd(true, true)?;

    // Count non-zero singular values (with tolerance for numerical precision)
    let tolerance = 1e-10;
    let rank = s.iter().filter(|&&value| value.abs() > tolerance).count();

    Ok(rank)
}
