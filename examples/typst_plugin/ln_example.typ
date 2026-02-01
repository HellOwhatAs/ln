#set page(margin: 0pt, height: auto)
#let ln = plugin("./ln_typst_plugin.wasm")
#import "@preview/suiji:0.5.1"

#image(ln.render(
  cbor.encode((
    eye: (3.75, 1.75, 2.0),
    center: (0.0, 0.0, 0.0),
    up: (0.0, 0.0, 1.0),
    width: 1024.0,
    height: 1024.0,
    fovy: 50.0,
    near: 0.1,
    far: 30.0,
    step: 0.01,
  )),
  cbor.encode((
    (
      Difference: (
        (
          Cube: (
            min: (0.0, 0.0, 0.0),
            max: (1.0, 1.0, 1.0),
          ),
        ),
        (
          Sphere: (
            center: (1.1, 1.1, 1.0),
            radius: 0.6,
            texture: "LatLng",
          ),
        ),
      ),
    ),
    (
      Sphere: (
        center: (2.0, 0.25, 0.5),
        radius: 0.6,
        texture: "RandomCircles",
        seed: 42,
      ),
    ),
  )),
))
