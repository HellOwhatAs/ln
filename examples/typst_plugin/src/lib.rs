pub mod constructor;

use ln::{Cube, Scene, Vector};
use wasm_minimal_protocol::*;
// Only necessary when using cbor for passing arguments.
use ciborium::de::from_reader;

initiate_protocol!();

#[derive(serde::Deserialize)]
struct RenderArgs {
    eye: [f64; 3],
    center: [f64; 3],
    up: [f64; 3],
    width: f64,
    height: f64,
    fovy: f64,
    near: f64,
    far: f64,
    step: f64,
}

#[wasm_func]
fn render(render_args: &[u8], shapes: &[u8]) -> Result<Vec<u8>, String> {
    let args: RenderArgs = from_reader(render_args).map_err(|e| e.to_string())?;
    let shapes: Vec<constructor::LnShape> = from_reader(shapes).map_err(|e| e.to_string())?;

    Ok(constructor::render(
        shapes.into_iter(),
        args.eye,
        args.center,
        args.up,
        args.width,
        args.height,
        args.fovy,
        args.near,
        args.far,
        args.step,
    )?
    .to_svg(args.width, args.height)
    .into_bytes())
}

#[wasm_func]
fn render0(render_args: &[u8], items: &[u8]) -> Result<Vec<u8>, String> {
    let args: RenderArgs = from_reader(render_args).map_err(|e| e.to_string())?;
    let mut scene = Scene::new();

    let shapes: Vec<[[f64; 3]; 2]> = from_reader(items).map_err(|e| e.to_string())?;
    for shape in shapes {
        let min = Vector::new(shape[0][0], shape[0][1], shape[0][2]);
        let max = Vector::new(shape[1][0], shape[1][1], shape[1][2]);
        let cube = Cube::new(min, max);
        scene.add(cube);
    }

    let paths = scene.render(
        Vector::new(args.eye[0], args.eye[1], args.eye[2]),
        Vector::new(args.center[0], args.center[1], args.center[2]),
        Vector::new(args.up[0], args.up[1], args.up[2]),
        args.width,
        args.height,
        args.fovy,
        args.near,
        args.far,
        args.step,
    );
    Ok(paths.to_svg(args.width, args.height).into_bytes())
}
