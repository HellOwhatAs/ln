use ln::{OutlineSphere, Scene, Vector, Sphere};

fn main() {
    let eye = Vector::new(8.0, 8.0, 8.0);
    let center = Vector::new(0.0, 0.0, 0.0);
    let up = Vector::new(0.0, 0.0, 1.0);

    let mut scene = Scene::new();

    // Add a single outline sphere at origin
    let sphere = OutlineSphere::new(eye, up, Vector::new(0.0, 0.0, 0.0), 1.0);
    scene.add(sphere);

    let width = 512.0;
    let height = 512.0;
    let fovy = 50.0;

    let paths = scene.render(eye, center, up, width, height, fovy, 0.1, 100.0, 0.01);
    paths.write_to_png("single_sphere.png", width, height);
    println!("Single sphere written to single_sphere.png");
    
    // Now test with two overlapping spheres
    let mut scene2 = Scene::new();
    
    // Two spheres, one slightly in front of the other
    let sphere1 = OutlineSphere::new(eye, up, Vector::new(0.0, 0.0, 0.0), 1.0);
    let sphere2 = OutlineSphere::new(eye, up, Vector::new(0.5, 0.5, 0.5), 1.0);
    scene2.add(sphere1);
    scene2.add(sphere2);

    let paths2 = scene2.render(eye, center, up, width, height, fovy, 0.1, 100.0, 0.01);
    paths2.write_to_png("two_spheres.png", width, height);
    println!("Two spheres written to two_spheres.png");
}
