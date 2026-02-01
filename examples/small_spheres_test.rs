use ln::{OutlineSphere, Scene, Vector};

fn main() {
    let eye = Vector::new(8.0, 8.0, 8.0);
    let center = Vector::new(0.0, 0.0, 0.0);
    let up = Vector::new(0.0, 0.0, 1.0);

    let mut scene = Scene::new();

    // Add several small overlapping spheres in a line (like beads)
    for i in 0..20 {
        let x = i as f64 * 0.05 - 0.5;  // Small step
        let sphere = OutlineSphere::new(eye, up, Vector::new(x, x, x), 0.1);  // Radius 0.1 like beads
        scene.add(sphere);
    }

    let width = 512.0;
    let height = 512.0;
    let fovy = 50.0;

    let paths = scene.render(eye, center, up, width, height, fovy, 0.1, 100.0, 0.01);
    paths.write_to_png("small_spheres.png", width, height);
    println!("Small spheres written to small_spheres.png");
}
