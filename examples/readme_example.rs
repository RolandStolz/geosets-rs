use geosets_rs::{HPolytope, sets::GeoSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let set = HPolytope::from_random(2, 4).unwrap();

    // Convert to vertices
    let _vertices = set.to_vertices().unwrap();

    // Calculate volume
    let volume = set.volume().unwrap();
    println!("Volume: {}", volume);

    // Translate the set
    let translation = ndarray::Array1::ones(2);
    let translated = set.translate(&translation).unwrap();

    // Plot
    let mut plot = translated.plot((0, 1), true, true).unwrap();

    // Plot with original
    let original_trace = set
        .create_trace((0, 1), Some("Original VPolytope"))
        .unwrap();
    plot.add_trace(original_trace);
    plot.show();

    Ok(())
}
