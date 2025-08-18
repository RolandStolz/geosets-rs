use geosets_rs::sets::GeoSet;
use geosets_rs::{HPolytope, VPolytope, Zonotope};
use ndarray::{array, Array2};
use plotly::common::Mode;
use plotly::layout::Layout;
use plotly::{Plot, Scatter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating and plotting geometric sets...");

    // Create different types of sets
    let set = VPolytope::from_unit_box(2);
    // plot_multiple_sets(vec![&zonotope, &unit_box_zonotope])?;
    set.plot((0, 1), true).unwrap();
    Ok(())
}
