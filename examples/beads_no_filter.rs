use ln::{OutlineSphere, Vector, Matrix, Paths, Shape};
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;

fn normalize(values: &[f64], a: f64, b: f64) -> Vec<f64> {
    let lo = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    values.iter().map(|&x| {
        let p = (x - lo) / (hi - lo);
        a + p * (b - a)
    }).collect()
}

fn low_pass(values: &[f64], alpha: f64) -> Vec<f64> {
    let mut result = Vec::with_capacity(values.len());
    let mut y = 0.0;
    for &x in values { y -= alpha * (y - x); result.push(y); }
    result
}

fn low_pass_noise(rng: &mut SmallRng, n: usize, alpha: f64, iterations: usize) -> Vec<f64> {
    let mut result: Vec<f64> = (0..n).map(|_| rng.gen()).collect();
    for _ in 0..iterations { result = low_pass(&result, alpha); }
    normalize(&result, -1.0, 1.0)
}

fn main() {
    let mut rng = SmallRng::seed_from_u64(1211);
    let eye = Vector::new(8.0, 8.0, 8.0);
    let center = Vector::new(0.0, 0.0, 0.0);
    let up = Vector::new(0.0, 0.0, 1.0);

    // Collect all paths WITHOUT visibility filtering
    let mut all_paths = Paths::new();
    
    for _ in 0..10 {  // Fewer strands for clarity
        let n = 50;   // Fewer spheres
        let xs = low_pass_noise(&mut rng, n, 0.3, 4);
        let ys = low_pass_noise(&mut rng, n, 0.3, 4);
        let zs = low_pass_noise(&mut rng, n, 0.3, 4);
        let ss = low_pass_noise(&mut rng, n, 0.3, 4);

        let mut position = Vector::new(0.0, 0.0, 0.0);
        for i in 0..n {
            let sphere = OutlineSphere::new(eye, up, position, 0.1);
            all_paths.extend(sphere.paths());
            let s = (ss[i] + 1.0) / 2.0 * 0.1 + 0.01;
            let v = Vector::new(xs[i], ys[i], zs[i]).normalize().mul_scalar(s);
            position = position.add(v);
        }
    }

    let width = 380.0 * 5.0;
    let height = 315.0 * 5.0;
    let fovy = 50.0;

    // Transform to 2D without filtering
    let aspect = width / height;
    let matrix = Matrix::look_at(eye, center, up);
    let matrix = matrix.with_perspective(fovy, aspect, 0.1, 100.0);
    
    // Transform paths to screen space
    let matrix2 = Matrix::translate(Vector::new(1.0, 1.0, 0.0))
        .scaled(Vector::new(width / 2.0, height / 2.0, 0.0));
    let combined = matrix2.mul(&matrix);
    
    let mut transformed_paths = Vec::new();
    for path in &all_paths.paths {
        let new_path: Vec<Vector> = path.iter()
            .map(|v| combined.mul_position_w(*v))
            .collect();
        transformed_paths.push(new_path);
    }
    
    let paths = Paths::from_vec(transformed_paths);
    paths.write_to_png("beads_no_filter.png", width, height);
    println!("Written beads_no_filter.png (no visibility filtering)");
}
