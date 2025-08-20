use geosets_rs::sets::GeoSet;
use geosets_rs::VPolytope;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let set = VPolytope::from_unit_box(3);
    
    // Get the vertices to see what we're working with
    let vertices = set.to_vertices()?;
    println!("Vertices:");
    for (i, vertex) in vertices.outer_iter().enumerate() {
        println!("  {}: {:?}", i, vertex);
    }
    println!("Number of vertices: {}", vertices.nrows());
    println!("Dimension: {}", vertices.ncols());
    
    let volume = set.volume()?;
    println!("Volume: {}", volume);

    Ok(())
}
