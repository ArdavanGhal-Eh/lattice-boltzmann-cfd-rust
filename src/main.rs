use lbm_solver::{LbmSolver, FlowSnapshot};
use std::time::Instant;

fn main() {
    println!("============================================================");
    println!("🌪️ 2D Lattice Boltzmann Flow Solver (LBM D2Q9) in Rust");
    println!("    Author: Ardavan Ghal-Eh | Sharif University of Tech");
    println!("============================================================");

    let nx = 200;
    let ny = 80;
    let tau = 0.56;        // Viscosity nu = (tau - 0.5)/3 = 0.02
    let u_inlet = 0.08;    // Inlet Mach number (well within incompressible limit)
    let cylinder_radius = 8.0;

    let reynolds = (u_inlet * (2.0 * cylinder_radius)) / ((tau - 0.5) / 3.0);
    println!("Mesh Grid: {} x {}", nx, ny);
    println!("Reynolds Number: Re = {:.1}", reynolds);
    println!("Relaxation Parameter (tau): {:.2}", tau);
    println!("Cylinder Diameter: {:.1} lu", 2.0 * cylinder_radius);
    println!("------------------------------------------------------------");

    let mut solver = LbmSolver::new(nx, ny, tau, u_inlet, cylinder_radius);
    let total_steps = 1500;
    let start_time = Instant::now();

    let mut snapshots = Vec::new();

    for step in 1..=total_steps {
        let (fx, fy) = solver.collide_and_stream();
        let (cd, cl) = solver.compute_coefficients(fx, fy);

        if step % 300 == 0 || step == total_steps {
            println!(
                "Step {:4}/{} | Drag Cd: {:6.3} | Lift Cl: {:+6.3} | Fx: {:.4} | Fy: {:.4}",
                step, total_steps, cd, cl, fx, fy
            );
            snapshots.push(FlowSnapshot {
                step,
                drag_force: fx,
                lift_force: fy,
                drag_coeff: cd,
                lift_coeff: cl,
                avg_density: 1.0,
            });
        }
    }

    let elapsed = start_time.elapsed();
    let mlups = (total_steps as f64 * (nx * ny) as f64) / (elapsed.as_secs_f64() * 1e6);
    println!("------------------------------------------------------------");
    println!("⏱️ Execution Time: {:.2?}", elapsed);
    println!("🚀 Lattice Updates/Sec: {:.2} MLUPS (Million Lattice Updates/Sec)", mlups);
    println!("============================================================");
}
