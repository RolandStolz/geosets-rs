use geosets_rs::sets::GeoSet;
use geosets_rs::{HPolytope, Interval};
use ndarray::array;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating and plotting geometric sets...");

    // Create different types of sets
    // let set = HPolytope::from_unit_box(2);
    let set = HPolytope::from_random(2, 5)?;
    let interval = Interval::new(array![-0.8, -0.8], array![0.5, 0.3]).unwrap();

    println!("{:?}", set.to_vertices()?);

    let mut plot = set.plot((0, 1), false).unwrap();
    let trace = interval.create_trace((0, 1)).unwrap();
    plot.add_trace(trace);
    plot.show();
    Ok(())
}
