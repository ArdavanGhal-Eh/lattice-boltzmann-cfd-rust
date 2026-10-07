use lattice_boltzmann_cfd_rust::{
    equilibrium, NacaAirfoil, LbmSolver, ThermalLbmSolver, WEIGHTS, CX, CY, NUM_DIRECTIONS
};

#[test]
fn test_d2q9_weights_and_moments() {
    let weight_sum: f64 = WEIGHTS.iter().sum();
    assert!((weight_sum - 1.0).abs() < 1e-6);

    let mut first_moment_x = 0.0;
    let mut first_moment_y = 0.0;
    for i in 0..NUM_DIRECTIONS {
        first_moment_x += WEIGHTS[i] * (CX[i] as f64);
        first_moment_y += WEIGHTS[i] * (CY[i] as f64);
    }
    assert!(first_moment_x.abs() < 1e-6);
    assert!(first_moment_y.abs() < 1e-6);
}

#[test]
fn test_equilibrium_conservation() {
    let rho = 1.25;
    let ux = 0.10;
    let uy = -0.05;

    let mut sum_feq = 0.0;
    let mut sum_feq_mx = 0.0;
    let mut sum_feq_my = 0.0;

    for i in 0..NUM_DIRECTIONS {
        let feq = equilibrium(rho, ux, uy, i);
        sum_feq += feq;
        sum_feq_mx += feq * (CX[i] as f64);
        sum_feq_my += feq * (CY[i] as f64);
    }

    assert!((sum_feq - rho).abs() < 1e-5);
    assert!((sum_feq_mx - rho * ux).abs() < 1e-5);
    assert!((sum_feq_my - rho * uy).abs() < 1e-5);
}

#[test]
fn test_lbm_solver_step_stability() {
    let nx = 40;
    let ny = 20;
    let tau = 0.8;
    let u_inlet = 0.05;
    let radius = 3.0;

    let mut solver = LbmSolver::new(nx, ny, tau, u_inlet, radius);

    for _ in 0..10 {
        let (fx, fy) = solver.collide_and_stream();
        assert!(!fx.is_nan());
        assert!(!fy.is_nan());
    }
    assert_eq!(solver.step_count, 10);
}

#[test]
fn test_naca_airfoil_geometry() {
    let chord = 1.0;
    let airfoil = NacaAirfoil::naca_0012(chord, 4.0);

    // Leading edge thickness is 0
    let yt_0 = airfoil.half_thickness(0.0);
    assert!(yt_0.abs() < 1e-4);

    // Peak thickness around 30% chord should be approx 0.06 (half of 12% chord)
    let yt_peak = airfoil.half_thickness(0.30);
    assert!((yt_peak - 0.06).abs() < 0.005);

    // Aerodynamic efficiency check
    let (l_d, status) = airfoil.compute_aerodynamic_efficiency(0.45, 0.015);
    assert!((l_d - 30.0).abs() < 1e-3);
    assert!(status.contains("Linear Lift"));
}

#[test]
fn test_thermal_solver_step() {
    let nx = 30;
    let ny = 15;
    let mut thermal = ThermalLbmSolver::new(nx, ny, 0.9, 20.0);
    let vx = vec![0.02; nx * ny];
    let vy = vec![0.0; nx * ny];
    let obstacles = vec![false; nx * ny];

    thermal.step_thermal(&vx, &vy, 80.0, &obstacles);

    for &t in &thermal.temperature {
        assert!(!t.is_nan());
        assert!(t >= 19.0 && t <= 85.0);
    }
}
