//! A library for multidimensional interpolation.

use nalgebra::{DMatrix, DVector};

pub enum Basis {
    PolyHarmonic(i32),
    Gaussian(f64),
}

pub struct Scatter {
    // Note: could make basis a type-level parameter
    basis: Basis,
    // TODO(explore): use matrix & slicing instead (fewer allocs).
    // An array of n vectors each of size m. 
    centers: Vec<DVector<f64>>,
    // An m x n matrix, where n is the number of basis functions, and m is the number of coords.
    deltas: DMatrix<f64>,
}

impl Basis {
    fn eval(&self, r: f64) -> f64 {
        match self {
            Basis::PolyHarmonic(n) if n % 2 == 0 => {
                // Somewhat arbitrary but don't expect tiny nonzero values.
                if r < 1e-12 {
                    0.0
                } else {
                    r.powi(*n) * r.ln()
                }
            }
            Basis::PolyHarmonic(n) => r.powi(*n),
            // Note: it might be slightly more efficient to pre-recip c, but
            // let's keep code clean for now.
            Basis::Gaussian(c) => (-(r/c).powi(2)).exp(),
        }
    }
}

impl Scatter {
    pub fn eval(&self, coords: DVector<f64>) -> DVector<f64> {
        let basis = DVector::from_fn(self.centers.len(), |r, _c|
            self.basis.eval((&coords - &self.centers[r]).norm()));
        &self.deltas * basis
    }

    pub fn create(centers: Vec<DVector<f64>>, vals: Vec<DVector<f64>>, basis: Basis) -> Scatter {
        let n = centers.len();
        // n x m matrix. There's probably a better way to do this, ah well.
        let vals = DMatrix::from_columns(&vals).transpose();
        let mat = DMatrix::from_fn(n, n, |r, c|
            basis.eval((&centers[r] - &centers[c]).norm())
        );
        // TODO: plumb errors
        // Note: it's probably better to use a decomposition rather than actually inverting
        // the matrix, see https://www.johndcook.com/blog/2010/01/19/dont-invert-that-matrix/
        // But for now the code is optimized for clarity.
        // inv is an n x n matrix.
        let inv = mat.try_inverse().expect("non-invertible matrix");
        // Again, this transpose feels like I don't know what I'm doing.
        let deltas = (inv * vals).transpose();
        Scatter { basis, centers, deltas }
    }
}
