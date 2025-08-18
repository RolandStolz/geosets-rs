use ndarray::concatenate;
use ndarray::prelude::*;
use plotly::common::Mode;
use plotly::Trace;
use plotly::{Plot, Scatter};

use self::errors::SetOperationError;

pub mod errors;
pub mod hpolytope;
pub mod interval;
pub mod vpolytope;
pub mod zonotope;

pub trait GeoSet: Sized + Clone {
    fn dim(&self) -> usize;
    fn empty(&self) -> Result<bool, SetOperationError>;

    // Static function
    fn from_unit_box(dim: usize) -> Self;

    fn to_vertices(&self) -> Result<Array2<f64>, SetOperationError>;
    fn center(&self) -> Result<Array1<f64>, SetOperationError>;
    fn support_function(&self) -> Result<(Array1<f64>, f64), SetOperationError>;
    fn volume(&self) -> Result<f64, SetOperationError>;

    // Operations
    fn minkowski_sum_(&self, other: &Self) -> Result<(), SetOperationError>;
    fn matmul_(&self, mat: &Array2<f64>) -> Result<(), SetOperationError>;
    fn translate_(&self, vector: &Array1<f64>) -> Result<(), SetOperationError>;

    fn minkowski_sum(&self, other: &Self) -> Result<Self, SetOperationError> {
        let copy = self.clone();
        copy.minkowski_sum_(other)?;
        Ok(copy)
    }
    fn matmul(&self, mat: &Array2<f64>) -> Result<Self, SetOperationError> {
        let copy = self.clone();
        copy.matmul_(mat)?;
        Ok(copy)
    }
    fn translate(&self, vector: &Array1<f64>) -> Result<Self, SetOperationError> {
        let copy = self.clone();
        copy.translate_(vector)?;
        Ok(copy)
    }

    // Generic implementations
    fn create_trace(&self, dim: (usize, usize)) -> Result<Box<dyn Trace>, SetOperationError> {
        use crate::geometric_operations::order_vertices_clockwise;
        let vertices = order_vertices_clockwise(self.to_vertices()?);

        let closed_vertices = ndarray::concatenate(
            Axis(0),
            &[vertices.view(), vertices.row(0).view().insert_axis(Axis(0))],
        )
        .unwrap();

        let x = closed_vertices.column(dim.0).to_vec();
        let y = closed_vertices.column(dim.1).to_vec();

        let trace = Scatter::new(x, y).mode(Mode::LinesMarkers);
        Ok(trace)
    }

    fn plot(&self, dim: (usize, usize), show: bool) -> Result<Plot, SetOperationError> {
        let mut plot = Plot::new();
        let trace = self.create_trace(dim).unwrap();
        plot.add_trace(trace);

        if show {
            plot.show();
        }

        Ok(plot)
    }
}
