use geosets_rs::sets::GeoSet;
use geosets_rs::{HPolytope, Interval, Zonotope};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating and plotting geometric sets...");

    // Create different types of sets
    // let set = HPolytope::from_unit_box(2);
    let interval = Interval::from_unit_box(2);
    let hpoly = HPolytope::from_random(2, 5)?;
    let zono = Zonotope::from_random(2, 6, true)?;

    println!("{:?}", hpoly.to_vertices()?);

    let mut plot = interval.plot((0, 1), true, false).unwrap();

    let hpoly_trace = hpoly.create_trace((0, 1), Some("H-Polytope")).unwrap();
    let zono_trace = zono.create_trace((0, 1), Some("Zonotope")).unwrap();

    plot.add_trace(hpoly_trace);
    plot.add_trace(zono_trace);
    plot.show();
    Ok(())
}
