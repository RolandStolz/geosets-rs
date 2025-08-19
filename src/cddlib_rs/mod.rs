mod cdd_bindings;

use crate::sets::errors::SetOperationError;
use cdd_bindings::*;
use ndarray::{Array1, Array2};
use std::os::raw::c_long;

pub fn compute_polytope_vertices(
    a: &Array2<f64>,
    b: &Array1<f64>,
) -> Result<Array2<f64>, SetOperationError> {
    // Check dimension compatibility
    if a.nrows() != b.len() {
        return Err(SetOperationError::DimensionMismatch {
            expected: a.nrows(),
            got: b.len(),
        });
    }

    let m = a.nrows() as c_long;
    let n = a.ncols() as c_long;

    unsafe {
        // Initialize cddlib
        dd_set_global_constants();

        // Create matrix [b | -A]
        let mat = dd_CreateMatrix(m, n + 1);
        if mat.is_null() {
            dd_free_global_constants();
            return Err(SetOperationError::DataConversionError {
                source: "Failed to create cddlib matrix".into(),
            });
        }

        (*mat).representation = dd_RepresentationType::dd_Inequality;

        for i in 0..m {
            let i_usize = i as usize;
            let i_isize = i as isize;

            // RHS - get row pointer, then column pointer, then access the [f64; 1] array
            let row_ptr = (*mat).matrix.offset(i_isize);
            let rhs_ptr = (*row_ptr).offset(0);
            (*rhs_ptr)[0] = b[i_usize];

            // Coefficients (-A)
            for j in 0..n {
                let j_usize = j as usize;
                let j_isize = (j + 1) as isize;
                let coeff_ptr = (*row_ptr).offset(j_isize);
                (*coeff_ptr)[0] = -a[[i_usize, j_usize]];
            }
        }

        // Build polyhedron
        let mut err: dd_ErrorType = dd_ErrorType::dd_NoError;
        let poly = dd_DDMatrix2Poly(mat, &mut err);
        if err != dd_ErrorType::dd_NoError || poly.is_null() {
            dd_FreeMatrix(mat);
            dd_free_global_constants();
            return Err(SetOperationError::InfeasibleOptimization {
                source: format!("cddlib error: {:?}", err).into(),
            });
        }

        // Extract generators (vertices + rays)
        let gens = dd_CopyGenerators(poly);
        if gens.is_null() {
            dd_FreePolyhedra(poly);
            dd_FreeMatrix(mat);
            dd_free_global_constants();
            return Err(SetOperationError::DataConversionError {
                source: "Failed to extract generators from polyhedron".into(),
            });
        }

        let mut vertices_data = Vec::new();
        let mut vertex_count = 0;

        for i in 0..(*gens).rowsize {
            let i_isize = i as isize;
            let gen_row_ptr = (*gens).matrix.offset(i_isize);
            let kind_ptr = (*gen_row_ptr).offset(0);
            let kind = (*kind_ptr)[0];

            if (kind - 1.0).abs() < 1e-9 {
                // It's a vertex
                for j in 1..=n {
                    let j_isize = j as isize;
                    let vertex_ptr = (*gen_row_ptr).offset(j_isize);
                    vertices_data.push((*vertex_ptr)[0]);
                }
                vertex_count += 1;
            }
        }

        // Cleanup
        dd_FreePolyhedra(poly);
        dd_FreeMatrix(mat);
        dd_FreeMatrix(gens);
        dd_free_global_constants();

        // Convert to Array2
        if vertex_count == 0 {
            Ok(Array2::zeros((0, n as usize)))
        } else {
            Array2::from_shape_vec((vertex_count, n as usize), vertices_data).map_err(|e| {
                SetOperationError::DataConversionError {
                    source: format!("Failed to create Array2 from vertices data: {}", e).into(),
                }
            })
        }
    }
}
