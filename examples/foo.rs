use nalgebra::DVector;

use scatter::{Basis, Scatter};

fn main() {
    let mut xs = Vec::with_capacity(10);
    let mut ys = Vec::with_capacity(10);
    for i in 0..10 {
        let x = 0.2 * (i as f64);
        let y = x.sin();
        xs.push(DVector::from_vec(vec![x]));
        ys.push(DVector::from_vec(vec![y]));
    }
    let scatter = Scatter::create(xs, ys, Basis::PolyHarmonic(3), 2);
    for i in 0..100 {
        let x = 0.02 * (i as f64);
        let y = scatter.eval(DVector::from_vec(vec![x]));
        println!("{} {} {}", x, y[0], x.sin());
    }
}
