use ln::{OutlineSphere, Scene, Vector, Shape, Ray};

fn main() {
    let eye = Vector::new(8.0, 8.0, 8.0);
    let up = Vector::new(0.0, 0.0, 1.0);
    
    // Single small sphere at origin
    let sphere = OutlineSphere::new(eye, up, Vector::new(0.0, 0.0, 0.0), 0.1);
    
    // Get the paths (silhouette circle)
    let paths = sphere.paths();
    
    // Check a few points on the silhouette
    println!("Sphere center: (0, 0, 0), radius: 0.1");
    println!("Eye: {:?}", eye);
    
    let first_path = &paths.paths[0];
    println!("Number of points on silhouette: {}", first_path.len());
    
    // Check first few points
    for (i, point) in first_path.iter().take(5).enumerate() {
        // Distance from point to sphere center
        let dist_to_center = point.sub(Vector::new(0.0, 0.0, 0.0)).length();
        println!("\nPoint {}: {:?}", i, point);
        println!("  Distance to sphere center: {} (should be ~0.1)", dist_to_center);
        
        // Shoot ray from point toward eye
        let v = eye.sub(*point);
        let ray_dir = v.normalize();
        let ray = Ray::new(*point, ray_dir);
        
        // Check intersection with the sphere
        let hit = sphere.intersect(ray);
        println!("  Ray toward eye - hit.t: {}, hit.ok: {}", hit.t, hit.ok);
        println!("  Distance to eye: {}", v.length());
        println!("  Visible (hit.t >= dist): {}", hit.t >= v.length());
    }
}
