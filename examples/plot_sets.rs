use geosets_rs::sets::GeoSet;
use geosets_rs::{HPolytope, Interval, VPolytope, Zonotope};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating and plotting geometric sets...");

    // Create different types of sets
    // let set = HPolytope::from_unit_box(2);
    let bbox = Interval::from_unit_box(2);
    let hpoly = HPolytope::from_random(2, 5)?;
    let vpoly = VPolytope::from_random(2, 10)?;
    let zono = Zonotope::from_random(2, 5, true)?;
    let interval = Interval::from_random(2)?;

    println!("{}", zono.n_generators());

    let mut plot = bbox.plot((0, 1), true, false).unwrap();

    let hpoly_trace = hpoly.create_trace((0, 1), Some("H-Polytope")).unwrap();
    let zono_trace = zono.create_trace((0, 1), Some("Zonotope")).unwrap();
    let interval_trace = interval.create_trace((0, 1), Some("Interval")).unwrap();
    let vpoly_trace = vpoly.create_trace((0, 1), Some("V-Polytope")).unwrap();

    plot.add_trace(hpoly_trace);
    plot.add_trace(zono_trace);
    plot.add_trace(interval_trace);
    plot.add_trace(vpoly_trace);

    println!("H-Polytope volume: {}", hpoly.volume()?);
    println!("Zonotope volume: {}", zono.volume()?);
    println!("Interval volume: {}", interval.volume()?);
    println!("V-Polytope volume: {}", vpoly.volume()?);

    plot.show();
    Ok(())
}
