use ln::{OutlineSphere, Scene, Vector};

fn main() {
    let eye = Vector::new(8.0, 8.0, 8.0);
    let center = Vector::new(0.0, 0.0, 0.0);
    let up = Vector::new(0.0, 0.0, 1.0);

    let mut scene = Scene::new();

    // Side by side spheres
    scene.add(OutlineSphere::new(eye, up, Vector::new(0.0, 0.0, 0.0), 0.1));
    scene.add(OutlineSphere::new(eye, up, Vector::new(0.15, -0.15, 0.0), 0.1));
    
    // Also add a few more around
    scene.add(OutlineSphere::new(eye, up, Vector::new(-0.15, 0.15, 0.0), 0.1));
    scene.add(OutlineSphere::new(eye, up, Vector::new(0.0, 0.0, 0.2), 0.1));

    let width = 512.0;
    let height = 512.0;
    let paths = scene.render(eye, center, up, width, height, 50.0, 0.1, 100.0, 0.01);
    paths.write_to_png("sidebyside.png", width, height);
    println!("Written to sidebyside.png");
}
