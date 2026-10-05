// D2Q9 Lattice Boltzmann constants and equilibrium functions
// Author: Ardavan Ghal-Eh | Sharif University of Technology

pub const NUM_DIRECTIONS: usize = 9;

// Discrete velocity vectors (cx, cy)
pub const CX: [i32; NUM_DIRECTIONS] = [0, 1, 0, -1, 0, 1, -1, -1, 1];
pub const CY: [i32; NUM_DIRECTIONS] = [0, 0, 1, 0, -1, 1, 1, -1, -1];

// Opposite direction indices for bounce-back boundary condition
pub const OPPOSITE: [usize; NUM_DIRECTIONS] = [0, 3, 4, 1, 2, 7, 8, 5, 6];

// Lattice weights for D2Q9
pub const WEIGHTS: [f64; NUM_DIRECTIONS] = [
    4.0 / 9.0,
    1.0 / 9.0,
    1.0 / 9.0,
    1.0 / 9.0,
    1.0 / 9.0,
    1.0 / 36.0,
    1.0 / 36.0,
    1.0 / 36.0,
    1.0 / 36.0,
];

#[inline(always)]
pub fn equilibrium(rho: f64, ux: f64, uy: f64, i: usize) -> f64 {
    let ci_u = (CX[i] as f64) * ux + (CY[i] as f64) * uy;
    let u_sq = ux * ux + uy * uy;
    WEIGHTS[i] * rho * (1.0 + 3.0 * ci_u + 4.5 * ci_u * ci_u - 1.5 * u_sq)
}
