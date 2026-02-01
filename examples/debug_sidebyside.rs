use ln::{OutlineSphere, Scene, Vector, Shape, Ray, Tree};
use std::sync::Arc;

fn main() {
    let eye = Vector::new(8.0, 8.0, 8.0);
    let up = Vector::new(0.0, 0.0, 1.0);
    
    // Create two spheres side by side (perpendicular to view direction)
    // View direction is (1, 1, 1), so perpendicular would be something like (1, -1, 0)
    let sphere1 = OutlineSphere::new(eye, up, Vector::new(0.0, 0.0, 0.0), 0.1);
    let sphere2 = OutlineSphere::new(eye, up, Vector::new(0.15, -0.15, 0.0), 0.1);  // Side by side
    
    // Build a tree for intersection testing
    let shapes: Vec<Arc<dyn Shape + Send + Sync>> = vec![
        Arc::new(sphere1.clone()),
        Arc::new(sphere2.clone()),
    ];
    let tree = Tree::new(shapes);
    
    println!("Testing two side-by-side spheres:");
    println!("Sphere1 center: (0, 0, 0), radius: 0.1");
    println!("Sphere2 center: (0.15, -0.15, 0), radius: 0.1");
    println!("Eye: {:?}", eye);
    
    // Get paths from sphere1
    let paths1 = sphere1.paths();
    let first_path = &paths1.paths[0];
    
    println!("\nChecking sphere1's silhouette points:");
    
    let mut visible_count = 0;
    let mut hidden_count = 0;
    
    for (i, point) in first_path.iter().enumerate() {
        // Shoot ray from point toward eye
        let v = eye.sub(*point);
        let ray_dir = v.normalize();
        let ray = Ray::new(*point, ray_dir);
        
        // Check intersection with ALL spheres via tree
        let hit = tree.intersect(ray);
        let dist_to_eye = v.length();
        let visible = hit.t >= dist_to_eye;
        
        if visible {
            visible_count += 1;
        } else {
            hidden_count += 1;
            if hidden_count <= 5 {
                println!("Point {} at {:?} - HIDDEN", i, point);
                println!("  hit.t: {}, dist_to_eye: {}", hit.t, dist_to_eye);
                
                let hit1 = sphere1.intersect(ray);
                let hit2 = sphere2.intersect(ray);
                println!("  sphere1 hit.t: {}", hit1.t);
                println!("  sphere2 hit.t: {}", hit2.t);
            }
        }
    }
    
    println!("\nSphere1 Total: {} visible, {} hidden out of {} points", 
             visible_count, hidden_count, first_path.len());
    
    // Check sphere2
    let paths2 = sphere2.paths();
    let second_path = &paths2.paths[0];
    
    visible_count = 0;
    hidden_count = 0;
    
    for point in second_path.iter() {
        let v = eye.sub(*point);
        let ray_dir = v.normalize();
        let ray = Ray::new(*point, ray_dir);
        
        let hit = tree.intersect(ray);
        let dist_to_eye = v.length();
        let visible = hit.t >= dist_to_eye;
        
        if visible {
            visible_count += 1;
        } else {
            hidden_count += 1;
        }
    }
    
    println!("Sphere2 Total: {} visible, {} hidden out of {} points", 
             visible_count, hidden_count, second_path.len());
}
