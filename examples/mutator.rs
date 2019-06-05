//! A reproduction of the example from the MutatorMath repo.
use nalgebra::DVector;

use scatter::{Basis, Scatter};

const SAMPLES: &[([usize; 2], [u8; 3])] = &[
    ([11, 0], [128, 128, 128]),
    ([14, 2], [0, 0, 127]),
    ([6, 6], [51, 51, 51]),
    ([9, 6], [0, 0, 127]),
    ([12, 6], [129, 63, 0]),
    ([6, 8], [51, 51, 51]),
    ([14, 16], [129, 63, 0]),
];

fn main() {
    let locs = SAMPLES
        .iter()
        .map(|(loc, _color)| DVector::from_vec(vec![loc[0] as f64 + 0.5, loc[1] as f64 + 0.5]))
        .collect::<Vec<_>>();
    let colors = SAMPLES
        .iter()
        .map(|(_loc, color)| DVector::from_vec(vec![color[0] as f64, color[1] as f64, color[2] as f64]))
        .collect::<Vec<_>>();
    let scatter = Scatter::create(locs, colors, Basis::PolyHarmonic(2), 2);
    let width = 460;
    let height = 460;
    // Just output ASCII PPM so we don't need to depend on image coders.
    println!("P3");
    println!("{} {}", width, height);
    println!("255");
    for y in 0..height {
        for x in 0..width {
            let u = (x as f64) * 0.05;
            let v = (y as f64) * 0.05;
            let interp = scatter.eval(DVector::from_vec(vec![u, v]));
            let r = interp[0].round().max(0.0).min(255.0) as u8;
            let g = interp[1].round().max(0.0).min(255.0) as u8;
            let b = interp[2].round().max(0.0).min(255.0) as u8;
            println!("{} {} {}", r, g, b);
        }
    }
}
