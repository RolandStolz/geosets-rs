use ndarray::concatenate;
use ndarray::prelude::*;
use plotly::Layout;
use plotly::Trace;
use plotly::common::Mode;
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
    fn degenerate(&self) -> bool;

    // Static function
    fn from_unit_box(dim: usize) -> Self;

    fn to_vertices(&self) -> Result<Array2<f64>, SetOperationError>;
    fn center(&self) -> Result<Array1<f64>, SetOperationError>;
    fn support_function(
        &self,
        direction: Array1<f64>,
    ) -> Result<(Array1<f64>, f64), SetOperationError>;
    fn volume(&self) -> Result<f64, SetOperationError>;
    fn contains_point(&self, point: &Array1<f64>) -> Result<bool, SetOperationError>;

    // Operations
    fn minkowski_sum_(&mut self, other: &Self) -> Result<(), SetOperationError>;
    fn matmul_(&mut self, mat: &Array2<f64>) -> Result<(), SetOperationError>;
    fn translate_(&mut self, vector: &Array1<f64>) -> Result<(), SetOperationError>;

    fn minkowski_sum(&self, other: &Self) -> Result<Self, SetOperationError> {
        let mut copy = self.clone();
        copy.minkowski_sum_(other)?;
        Ok(copy)
    }
    fn matmul(&self, mat: &Array2<f64>) -> Result<Self, SetOperationError> {
        let mut copy = self.clone();
        copy.matmul_(mat)?;
        Ok(copy)
    }
    fn translate(&self, vector: &Array1<f64>) -> Result<Self, SetOperationError> {
        let mut copy = self.clone();
        copy.translate_(vector)?;
        Ok(copy)
    }

    // Generic implementations
    fn create_trace(
        &self,
        dim: (usize, usize),
        name: Option<&str>,
    ) -> Result<Box<dyn Trace>, SetOperationError> {
        use crate::geometric_operations::order_vertices_clockwise;
        let full_vertices = self.to_vertices()?;
        let col_x = full_vertices.column(dim.0);
        let col_y = full_vertices.column(dim.1);
        let vertices_2d = ndarray::stack(Axis(1), &[col_x, col_y]).unwrap();

        let vertices = order_vertices_clockwise(vertices_2d).unwrap();

        let closed_vertices = ndarray::concatenate(
            Axis(0),
            &[vertices.view(), vertices.row(0).view().insert_axis(Axis(0))],
        )
        .unwrap();

        let x = closed_vertices.column(dim.0).to_vec();
        let y = closed_vertices.column(dim.1).to_vec();

        let mut trace = Scatter::new(x, y)
            .mode(Mode::LinesMarkers)
            .fill(plotly::common::Fill::ToSelf)
            .opacity(0.8);

        if let Some(trace_name) = name {
            trace = trace.name(trace_name);
        }

        Ok(trace)
    }

    fn plot(
        &self,
        dim: (usize, usize),
        equal_axis: bool,
        show: bool,
    ) -> Result<Plot, SetOperationError> {
        let mut plot = Plot::new();
        let trace = self.create_trace(dim, None).unwrap();
        plot.add_trace(trace);

        if equal_axis {
            let layout = Layout::new()
                .x_axis(plotly::layout::Axis::new())
                .y_axis(plotly::layout::Axis::new().scale_anchor("x"));
            plot.set_layout(layout);
        }

        if show {
            plot.show();
        }

        Ok(plot)
    }

    // Utils
    fn _check_operand_dim(&self, dim: usize) -> Result<(), SetOperationError> {
        if dim != self.dim() {
            return Err(SetOperationError::DimensionMismatch {
                expected: self.dim(),
                got: dim,
            });
        }
        Ok(())
    }
}
