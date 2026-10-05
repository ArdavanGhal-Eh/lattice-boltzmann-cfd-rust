use crate::lattice::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FlowSnapshot {
    pub step: usize,
    pub drag_force: f64,
    pub lift_force: f64,
    pub drag_coeff: f64,
    pub lift_coeff: f64,
    pub avg_density: f64,
}

pub struct LbmSolver {
    pub nx: usize,
    pub ny: usize,
    pub tau: f64,      // Relaxation time (controls kinematic viscosity)
    pub f: Vec<f64>,   // Distribution functions [ny, nx, 9]
    pub f_post: Vec<f64>,
    pub obstacles: Vec<bool>, // Solid mask [ny, nx]
    pub cylinder_center: (f64, f64),
    pub cylinder_radius: f64,
    pub u_inlet: f64,
    pub step_count: usize,
}

impl LbmSolver {
    pub fn new(nx: usize, ny: usize, tau: f64, u_inlet: f64, radius: f64) -> Self {
        let size = nx * ny * NUM_DIRECTIONS;
        let mut f = vec![0.0; size];
        let mut f_post = vec![0.0; size];
        let mut obstacles = vec![false; nx * ny];

        let cx = nx as f64 / 4.0;
        let cy = ny as f64 / 2.0;

        // Initialize obstacle mask (circular cylinder)
        for y in 0..ny {
            for x in 0..nx {
                let dx = x as f64 - cx;
                let dy = y as f64 - cy;
                if dx * dx + dy * dy <= radius * radius {
                    obstacles[y * nx + x] = true;
                }
            }
        }

        // Initialize flow field to uniform inlet velocity
        for y in 0..ny {
            for x in 0..nx {
                for i in 0..NUM_DIRECTIONS {
                    let idx = (y * nx + x) * NUM_DIRECTIONS + i;
                    let val = equilibrium(1.0, u_inlet, 0.0, i);
                    f[idx] = val;
                    f_post[idx] = val;
                }
            }
        }

        Self {
            nx,
            ny,
            tau,
            f,
            f_post,
            obstacles,
            cylinder_center: (cx, cy),
            cylinder_radius: radius,
            u_inlet,
            step_count: 0,
        }
    }

    #[inline(always)]
    fn idx(&self, x: usize, y: usize, i: usize) -> usize {
        (y * self.nx + x) * NUM_DIRECTIONS + i
    }

    pub fn collide_and_stream(&mut self) -> (f64, f64) {
        let omega = 1.0 / self.tau;
        let nx = self.nx;
        let ny = self.ny;
        let mut fx_drag = 0.0;
        let mut fy_lift = 0.0;

        // 1. Collision step
        for y in 0..ny {
            for x in 0..nx {
                let cell_idx = y * nx + x;
                if self.obstacles[cell_idx] {
                    continue;
                }

                // Compute macroscopic density and momentum
                let mut rho = 0.0;
                let mut mx = 0.0;
                let mut my = 0.0;

                for i in 0..NUM_DIRECTIONS {
                    let fi = self.f[self.idx(x, y, i)];
                    rho += fi;
                    mx += (CX[i] as f64) * fi;
                    my += (CY[i] as f64) * fi;
                }

                let inv_rho = 1.0 / rho;
                let ux = mx * inv_rho;
                let uy = my * inv_rho;

                // BGK Relaxation
                for i in 0..NUM_DIRECTIONS {
                    let feq = equilibrium(rho, ux, uy, i);
                    let fi = self.f[self.idx(x, y, i)];
                    self.f_post[self.idx(x, y, i)] = fi - omega * (fi - feq);
                }
            }
        }

        // 2. Streaming step with Momentum Exchange on Obstacles
        for y in 0..ny {
            for x in 0..nx {
                let cell_idx = y * nx + x;
                if self.obstacles[cell_idx] {
                    continue;
                }

                for i in 0..NUM_DIRECTIONS {
                    let src_x = (x as i32 - CX[i] + nx as i32) as usize % nx;
                    let src_y = (y as i32 - CY[i] + ny as i32) as usize % ny;
                    let src_cell = src_y * nx + src_x;

                    if self.obstacles[src_cell] {
                        // Bounce-back boundary condition
                        let opp = OPPOSITE[i];
                        let val = self.f_post[self.idx(x, y, opp)];
                        self.f[self.idx(x, y, i)] = val;

                        // Momentum exchange force calculation
                        fx_drag += (CX[i] as f64) * (self.f_post[self.idx(x, y, opp)] + val);
                        fy_lift += (CY[i] as f64) * (self.f_post[self.idx(x, y, opp)] + val);
                    } else {
                        self.f[self.idx(x, y, i)] = self.f_post[self.idx(src_x, src_y, i)];
                    }
                }

                // Inflow boundary (Dirichlet velocity on left wall)
                if x == 0 {
                    for i in 0..NUM_DIRECTIONS {
                        self.f[self.idx(x, y, i)] = equilibrium(1.0, self.u_inlet, 0.0, i);
                    }
                }
            }
        }

        self.step_count += 1;
        (fx_drag, fy_lift)
    }

    pub fn compute_coefficients(&self, fx: f64, fy: f64) -> (f64, f64) {
        let diameter = 2.0 * self.cylinder_radius;
        let q = 0.5 * 1.0 * (self.u_inlet * self.u_inlet) * diameter;
        let cd = if q > 1e-9 { fx / q } else { 0.0 };
        let cl = if q > 1e-9 { fy / q } else { 0.0 };
        (cd, cl)
    }
}
