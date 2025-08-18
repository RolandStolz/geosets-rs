use geosets_rs::sets::GeoSet;
use geosets_rs::{Interval, VPolytope, Zonotope};
use ndarray::{array, Array2};
use plotly::common::Mode;
use plotly::layout::Layout;
use plotly::{Plot, Scatter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating and plotting geometric sets...");

    // Create different types of sets
    let set = VPolytope::from_unit_box(2);
    let interval = Interval::new(array![-0.8, -0.8], array![0.5, 0.3]).unwrap();

    let mut plot = set.plot((0, 1), false).unwrap();
    let trace = interval.create_trace((0, 1)).unwrap();
    plot.add_trace(trace);
    plot.show();
    Ok(())
}
