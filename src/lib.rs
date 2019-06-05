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
    // An m x n' matrix, where n' is the number of basis functions (including polynomial),
    // and m is the number of coords.
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
            Basis::Gaussian(c) => (-(r / c).powi(2)).exp(),
        }
    }
}

impl Scatter {
    pub fn eval(&self, coords: DVector<f64>) -> DVector<f64> {
        let n = self.centers.len();
        let basis = DVector::from_fn(self.deltas.ncols(), |row, _c| {
            if row < n {
                // component from basis functions
                self.basis.eval((&coords - &self.centers[row]).norm())
            } else if row == n {
                // constant component
                1.0
            } else {
                // linear component
                coords[row - n - 1]
            }
        });
        &self.deltas * basis
    }

    // The order for the polynomial part, meaning terms up to (order - 1) are included.
    // This usage is consistent with Wilna du Toit's masters thesis "Radial Basis
    // Function Interpolation"
    pub fn create(
        centers: Vec<DVector<f64>>,
        vals: Vec<DVector<f64>>,
        basis: Basis,
        order: usize,
    ) -> Scatter {
        let n = centers.len();
        // n x m matrix. There's probably a better way to do this, ah well.
        let mut vals = DMatrix::from_columns(&vals).transpose();
        let n_aug = match order {
            0 => n,
            1 => n + 1,
            2 => n + 1 + centers[0].len(),
            _ => unimplemented!("don't yet support higher order polynomials"),
        };
        if n_aug > n {
            vals = vals.resize_vertically(n_aug, 0.0);
        }
        let mat = DMatrix::from_fn(n_aug, n_aug, |r, c| {
            if r < n && c < n {
                basis.eval((&centers[r] - &centers[c]).norm())
            } else if r < n {
                if c == n {
                    1.0
                } else {
                    centers[r][c - n - 1]
                }
            } else if c < n {
                if r == n {
                    1.0
                } else {
                    centers[c][r - n - 1]
                }
            } else {
                0.0
            }
        });
        // TODO: plumb errors
        // Note: it's probably better to use a decomposition rather than actually inverting
        // the matrix, see https://www.johndcook.com/blog/2010/01/19/dont-invert-that-matrix/
        // But for now the code is optimized for clarity.
        // inv is an n x n matrix.
        let inv = mat.try_inverse().expect("non-invertible matrix");
        // Again, this transpose feels like I don't know what I'm doing.
        let deltas = (inv * vals).transpose();
        Scatter {
            basis,
            centers,
            deltas,
        }
    }
}
