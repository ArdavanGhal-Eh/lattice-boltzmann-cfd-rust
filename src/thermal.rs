// Thermal Convection & Double-Population Energy Solver (TLBM D2Q9)
// Part of Lattice Boltzmann CFD Suite in Rust
// Author: Ardavan Ghal-Eh | Sharif University of Technology

use crate::lattice::*;

pub struct ThermalLbmSolver {
    pub nx: usize,
    pub ny: usize,
    pub tau_thermal: f64, // Thermal diffusivity alpha = (tau_T - 0.5)/3
    pub g: Vec<f64>,      // Thermal distribution functions [ny, nx, 9]
    pub g_post: Vec<f64>,
    pub temperature: Vec<f64>,
}

impl ThermalLbmSolver {
    pub fn new(nx: usize, ny: usize, tau_t: f64, t_ambient: f64) -> Self {
        let size = nx * ny * NUM_DIRECTIONS;
        let mut g = vec![0.0; size];
        let mut g_post = vec![0.0; size];
        let temperature = vec![t_ambient; nx * ny];

        for y in 0..ny {
            for x in 0..nx {
                for i in 0..NUM_DIRECTIONS {
                    let idx = (y * nx + x) * NUM_DIRECTIONS + i;
                    let eq = WEIGHTS[i] * t_ambient;
                    g[idx] = eq;
                    g_post[idx] = eq;
                }
            }
        }

        Self {
            nx,
            ny,
            tau_thermal: tau_t,
            g,
            g_post,
            temperature,
        }
    }

    #[inline(always)]
    fn idx(&self, x: usize, y: usize, i: usize) -> usize {
        (y * self.nx + x) * NUM_DIRECTIONS + i
    }

    // Thermal equilibrium function: g_i^eq = w_i * T * (1 + (e_i . u)/cs^2)
    #[inline(always)]
    pub fn thermal_equilibrium(t: f64, ux: f64, uy: f64, i: usize) -> f64 {
        let ci_u = (CX[i] as f64) * ux + (CY[i] as f64) * uy;
        WEIGHTS[i] * t * (1.0 + 3.0 * ci_u)
    }

    // Step thermal collision, convection streaming, and heated boundary update
    pub fn step_thermal(&mut self, velocity_x: &[f64], velocity_y: &[f64], t_hot_cylinder: f64, obstacles: &[bool]) {
        let omega_t = 1.0 / self.tau_thermal;
        let nx = self.nx;
        let ny = self.ny;

        // 1. Collision
        for y in 0..ny {
            for x in 0..nx {
                let cell = y * nx + x;
                let ux = velocity_x[cell];
                let uy = velocity_y[cell];
                let mut t_val = 0.0;

                for i in 0..NUM_DIRECTIONS {
                    t_val += self.g[self.idx(x, y, i)];
                }
                self.temperature[cell] = t_val;

                for i in 0..NUM_DIRECTIONS {
                    let geq = Self::thermal_equilibrium(t_val, ux, uy, i);
                    let gi = self.g[self.idx(x, y, i)];
                    self.g_post[self.idx(x, y, i)] = gi - omega_t * (gi - geq);
                }
            }
        }

        // 2. Streaming & Dirichlet heated cylinder wall
        for y in 0..ny {
            for x in 0..nx {
                let cell = y * nx + x;
                for i in 0..NUM_DIRECTIONS {
                    let src_x = (x as i32 - CX[i] + nx as i32) as usize % nx;
                    let src_y = (y as i32 - CY[i] + ny as i32) as usize % ny;
                    let src_cell = src_y * nx + src_x;

                    if obstacles[cell] {
                        // Heated obstacle surface (Dirichlet condition)
                        self.g[self.idx(x, y, i)] = Self::thermal_equilibrium(t_hot_cylinder, 0.0, 0.0, i);
                    } else if obstacles[src_cell] {
                        let opp = OPPOSITE[i];
                        self.g[self.idx(x, y, i)] = self.g_post[self.idx(x, y, opp)];
                    } else {
                        self.g[self.idx(x, y, i)] = self.g_post[self.idx(src_x, src_y, i)];
                    }
                }
            }
        }
    }
}
