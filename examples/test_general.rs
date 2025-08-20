use geosets_rs::sets::GeoSet;
use geosets_rs::VPolytope;
use ndarray::Array2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut set = VPolytope::from_unit_box(3);
    let volume = set.volume()?;
    println!("Volume of the VPolytope set: {}", volume);

    set.matmul_(&(Array2::eye(3) * 2.0)).unwrap();
    println!("{:?}", set.center());

    let volume = set.volume()?;
    println!("Volume after scaling: {}", volume);

    Ok(())
}
