#set page(margin: 0pt, height: auto)
#import "@preview/suiji:0.5.1"
#import "../lib.typ": *

#let rng = suiji.gen-rng-f(42)

#render(
  eye: (2., 7., 5.),
  center: (1.5, 2., 0.),
  step: 0.01,
  cube((0., 0., 0.), (1., 1., 1.)),
  cube((1.5, 0., 0.), (2.5, 1., 1.), texture: "Stripes", stripes: 8),
  sphere((0.5, 2., .5), 0.5),
  sphere((2., 2., .5), 0.5, texture: "RandomCircles"),
  sphere((0.5, 3.5, .5), 0.5, texture: "RandomEquators"),
  outline(sphere((2., 3.5, .5), 0.5)),
  cone(0.5, (-1., 0.5, 0.), (-1., 0.5, 1.)),
  translate(outline(cone(0.5, (0., 0., 0.), (0., 0., 1.))), (-1., 2.0, 0.)),
  cylinder(0.5, (3.5, 0.5, 0.), (3.5, 0.5, 1.)),
  translate(outline(cylinder(0.5, (0., 0., 0.), (0., 0., 1.))), (3.5, 2.0, 0.)),
)

#{
  let cubes = ()
  for x in range(-2, 3) {
    for y in range(-2, 3) {
      let z
      (rng, z) = suiji.random-f(rng)
      cubes.push(cube(
        (x - 0.5, y - 0.5, z - 0.5),
        (x + 0.5, y + 0.5, z + 0.5),
        texture: "Stripes",
        stripes: 7,
      ))
    }
  }
  render(step: 0.05, ..cubes)
}

#render(
  eye: (3., 2.5, 2.0),
  center: (1., 0.0, 0.0),
  step: 0.05,
  difference(
    cube((0., 0., 0.), (1., 1., 1.), texture: "Stripes", stripes: 15),
    sphere((1., 1., 0.5), 0.5),
    sphere((0., 1., 0.5), 0.5),
    sphere((0., 0., 0.5), 0.5),
    sphere((1., 0., 0.5), 0.5),
  ),
  cube((0.3, 0.3, 1.), (.7, .7, 1.2), texture: "Stripes", stripes: 10),
  intersection(
    sphere((2.0, 0.25, 0.5), 0.6),
    cube((1.5, -0.25, 0.), (2.5, 0.75, 1.0), texture: "Stripes", stripes: 25),
  ),
)

#{
  let n = 8
  let shapes = ()
  for x in range(-n, n + 1) {
    for y in range(-n, n + 2) {
      let seed
      (rng, seed) = suiji.integers-f(rng)
      shapes.push(
        outline(sphere(
          (float(x), float(y), 0.),
          0.45,
          texture: "RandomCircles",
          seed: seed,
        )),
      )
    }
  }
  render(
    eye: (8.0, 8.0, 1.0),
    center: (0., 0., -4.25),
    ..shapes,
  )
}


#{
  let n = 15
  let cubes = ()
  for x in range(-n, n + 1) {
    for y in range(-n, n + 1) {
      let p
      let fz
      (rng, (p, fz)) = suiji.random-f(rng, size: 2)
      let (fx, fy, fz, p) = (float(x), float(y), fz * 3 + 1, p * 0.25 + 0.2)
      if x == 2 and y == 1 {
        continue
      }
      cubes.push(
        cube(
          (fx - p, fy - p, 0.),
          (fx + p, fy + p, fz),
          texture: "Stripes",
          stripes: 7,
        ),
      )
    }
  }
  render(
    eye: (1.75, 1.25, 6.0),
    fovy: 100.0,
    ..cubes,
  )
}

#{
  let obj = read("./suzanne.obj")
  let float-pat = `[+-]?\d*(?:\.\d+)?`.text
  let v-pat = regex("v" + range(3).map(_ => "\s+(" + float-pat + ")").join())
  let vs = obj.matches(v-pat).map(x => x.captures.map(float))
  let fs = obj
    .split("\n")
    .map(str.trim)
    .filter(x => x.starts-with("f "))
    .map(x => {
      let vis = x.split().slice(1).map(int)
      range(1, vis.len() - 1).map(i => triangle(
        ..(0, i, i + 1).map(i => vs.at(vis.at(i) - 1)),
      ))
    })
    .flatten()
  render(
    eye: (2.5, 1.0, 6.0),
    center: (1.0, -0.5, 0.0),
    up: (0.0, 1.0, 0.0),
    fovy: 35.0,
    step: 0.01,
    ..fs,
  )
}

#{
  let nodes = (
    (1.047, -0.000, -1.312),
    (-0.208, -0.000, -1.790),
    (2.176, 0.000, -2.246),
    (1.285, -0.001, 0.016),
    (-1.276, -0.000, -0.971),
    (-0.384, 0.000, -2.993),
    (-2.629, -0.000, -1.533),
    (-1.098, -0.000, 0.402),
    (0.193, 0.005, 0.911),
    (-1.934, -0.000, 1.444),
    (2.428, -0.000, 0.437),
    (0.068, -0.000, 2.286),
    (-1.251, -0.000, 2.560),
    (1.161, -0.000, 3.261),
    (1.800, 0.001, -3.269),
    (2.783, 0.890, -2.082),
    (2.783, -0.889, -2.083),
    (-2.570, -0.000, -2.622),
    (-3.162, -0.890, -1.198),
    (-3.162, 0.889, -1.198),
    (-1.679, 0.000, 3.552),
    (1.432, -1.028, 3.503),
    (2.024, 0.513, 2.839),
    (0.839, 0.513, 4.167),
  )

  let edges = (
    (0, 1),
    (0, 2),
    (0, 3),
    (1, 4),
    (1, 5),
    (2, 14),
    (2, 15),
    (2, 16),
    (3, 8),
    (3, 10),
    (4, 6),
    (4, 7),
    (6, 17),
    (6, 18),
    (6, 19),
    (7, 8),
    (7, 9),
    (8, 11),
    (9, 12),
    (11, 12),
    (11, 13),
    (12, 20),
    (13, 21),
    (13, 22),
    (13, 23),
  )

  render(
    eye: (6., 6., 6.),
    step: 0.01,
    ..nodes.map(x => outline(sphere(x, 0.333))),
    ..edges.map(
      x => outline(cylinder(0.1, nodes.at(x.at(0)), nodes.at(x.at(1)))),
    ),
  )
}
