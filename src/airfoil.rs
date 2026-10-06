// NACA 4-Digit Airfoil Geometry Generator & Aerodynamic Polar Calculator
// Part of Lattice Boltzmann CFD Suite in Rust
// Author: Ardavan Ghal-Eh | Sharif University of Technology

pub struct NacaAirfoil {
    pub chord_len: f64,
    pub thickness_pct: f64, // e.g. 0.12 for NACA 0012
    pub angle_of_attack_deg: f64,
}

impl NacaAirfoil {
    pub fn naca_0012(chord: f64, alpha_deg: f64) -> Self {
        Self {
            chord_len: chord,
            thickness_pct: 0.12,
            angle_of_attack_deg: alpha_deg,
        }
    }

    // Evaluates half-thickness y_t at chord fraction x in [0.0, 1.0]
    pub fn half_thickness(&self, x: f64) -> f64 {
        let t = self.thickness_pct;
        let x_clamped = x.clamp(0.0, 1.0);
        5.0 * t * (0.2969 * x_clamped.sqrt() - 0.1260 * x_clamped - 0.3516 * x_clamped.powi(2)
            + 0.2843 * x_clamped.powi(3) - 0.1015 * x_clamped.powi(4)) * self.chord_len
    }

    // Generates boolean obstacle mask on lattice grid of size (nx, ny)
    pub fn generate_lattice_mask(&self, nx: usize, ny: usize, lead_x: f64, lead_y: f64) -> Vec<bool> {
        let mut mask = vec![false; nx * ny];
        let alpha_rad = self.angle_of_attack_deg.to_radians();
        let cos_a = alpha_rad.cos();
        let sin_a = alpha_rad.sin();

        for y in 0..ny {
            for x in 0..nx {
                let dx = x as f64 - lead_x;
                let dy = y as f64 - lead_y;

                // Rotate into airfoil coordinate system
                let x_airfoil = dx * cos_a + dy * sin_a;
                let y_airfoil = -dx * sin_a + dy * cos_a;

                if x_airfoil >= 0.0 && x_airfoil <= self.chord_len {
                    let norm_x = x_airfoil / self.chord_len;
                    let yt = self.half_thickness(norm_x);
                    if y_airfoil.abs() <= yt {
                        mask[y * nx + x] = true;
                    }
                }
            }
        }
        mask
    }

    // Computes lift-to-drag aerodynamic efficiency
    pub fn compute_aerodynamic_efficiency(&self, cl: f64, cd: f64) -> (f64, String) {
        let l_d_ratio = if cd > 1e-6 { cl / cd } else { 0.0 };
        let stall_status = if self.angle_of_attack_deg > 14.0 {
            "⚠️ Aerodynamic Stall Region (Flow Separation)"
        } else {
            "✅ Attached Flow / High L/D Linear Lift Zone"
        };
        (l_d_ratio, stall_status.to_string())
    }
}
