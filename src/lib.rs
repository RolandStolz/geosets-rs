#![doc = include_str!("../README.md")]
#![allow(rustdoc::broken_intra_doc_links)]

mod cddlib_rs;
mod geometric_operations;
mod linalg_utils;
mod qhull_wrapper;
pub mod sets;

pub use sets::GeoSet;
pub use sets::hpolytope::HPolytope;
pub use sets::interval::Interval;
pub use sets::vpolytope::VPolytope;
pub use sets::zonotope::Zonotope;

pub use cddlib_rs::compute_polytope_vertices;
pub use geometric_operations::order_vertices_clockwise;
