# 🌪️ Real-Time 2D Lattice Boltzmann (LBM D2Q9) Flow & Aerodynamic Drag Simulator (Rust + Python)

A high-performance computational fluid dynamics (CFD) solver developed in **Rust** utilizing the **Lattice Boltzmann Method (LBM D2Q9)** with the **Bhatnagar-Gross-Krook (BGK)** collision operator. Accurately simulates laminar vortex shedding (*von Kármán vortex street*) around immersed bluff bodies, featuring real-time aerodynamic drag ($C_d$) and lift ($C_l$) tracking via momentum exchange, paired with high-resolution Python vorticity visualization.

---

## 📌 Scientific & Industrial Context
Traditional CFD methods based on Navier-Stokes discretization (such as Finite Volume or Finite Element) require solving complex pressure Poisson equations at every timestep, demanding heavy linear algebra iterations.
The **Lattice Boltzmann Method (LBM)** models fluid flow through mesoscopic particle distribution functions colliding and streaming on a regular Cartesian grid:
1. **Zero Matrix Inversion:** Hyper-parallel, local collision operations achieving multi-million lattice updates per second (MLUPS).
2. **Exact Conservation:** Strictly preserves mass and linear momentum down to machine precision.
3. **Complex Geometry Ready:** Effortless handling of internal boundary bounce-back for airfoils, turbines, and porous media.

---

## 🌟 Architecture & Data Flow

```text
┌────────────────────────────────────────────────────────┐
│             Lattice Initialization (D2Q9)              │
│       Uniform Inlet Velocity, Obstacle Geometry        │
└──────────────────────────┬─────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│       1. BGK Collision Step (Local per node)           │
│  f_i^*(x, t) = f_i - (1/tau) * [ f_i - f_i^eq(rho, u) ]│
└──────────────────────────┬─────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│       2. Streaming & Bounce-Back Boundaries            │
│       - Periodic Top/Bottom Walls                      │
│       - Momentum Exchange on Solid Cylinder Boundary   │
└──────────────────────────┬─────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│       3. Aerodynamic Drag (Cd) & Lift (Cl) Output      │
│       Vorticity Contour & Streamline Generation        │
└────────────────────────────────────────────────────────┘
```

---

## 📐 Mathematical Formulation

### 1. D2Q9 Velocity Set & Equilibrium Distribution
The discrete velocities $\mathbf{e}_i$ and lattice weights $w_i$:
- Center ($i=0$): $\mathbf{e}_0 = (0, 0), \; w_0 = 4/9$
- Axial ($i=1..4$): $\mathbf{e}_i = (\pm 1, 0), (0, \pm 1), \; w_i = 1/9$
- Diagonal ($i=5..8$): $\mathbf{e}_i = (\pm 1, \pm 1), \; w_i = 1/36$

The Maxwell-Boltzmann equilibrium distribution:
$$f_i^{eq}(\rho, \mathbf{u}) = w_i \rho \left[ 1 + \frac{\mathbf{e}_i \cdot \mathbf{u}}{c_s^2} + \frac{(\mathbf{e}_i \cdot \mathbf{u})^2}{2 c_s^4} - \frac{|\mathbf{u}|^2}{2 c_s^2} \right]$$
*(where speed of sound $c_s = 1 / \sqrt{3}$)*

### 2. Kinematic Viscosity & Relaxation Time
$$\nu = c_s^2 \left( \tau - \frac{1}{2} \Delta t \right) = \frac{2\tau - 1}{6}$$

### 3. Momentum Exchange Aerodynamic Force
Force exerted by fluid on solid boundary nodes $\mathbf{x}_b$:
$$\mathbf{F} = \sum_{\mathbf{x}_b} \sum_{i} \mathbf{e}_i \left( f_i^*(\mathbf{x}_b) + f_{\bar{i}}(\mathbf{x}_b) \right)$$
$$C_d = \frac{2 F_x}{\rho_\infty U_\infty^2 D}, \quad C_l = \frac{2 F_y}{\rho_\infty U_\infty^2 D}$$

---

## 🎯 Real-World Applications & Cross-Industry Impact

### ⚙️ Mechanical & Aerospace Engineering
- **Automotive Aerodynamic Drag Optimization:** Predicting boundary layer separation, recirculation wakes, and aerodynamic drag coefficients ($C_d$) on vehicle chasses.
- **Turbomachinery & Wind Turbine Blade Stall:** Modeling unsteady vortex shedding on oscillating airfoils and evaluating aerodynamic flutter.
- **Cooling Flow in Electronic Enclosures:** Modeling laminar heat-sink airflow with low computational overhead.

### 🌐 Cross-Industry & Software Applications
- **Real-Time Physics Engines for Simulators:** High-fidelity fluid physics in defense training simulators, naval wave-body interaction, and visual effects (VFX).
- **Blood Flow & Microfluidics:** Simulating red blood cell suspensions in lab-on-a-chip capillary channels without continuous re-meshing.

---

## 🚀 Installation & Benchmark Execution

### 1. Build & Run Rust Computational Core
```bash
cargo build --release
cargo run --release
```

### 2. Run Python Vorticity & Aerodynamic Plotter
```bash
cd python_visualizer
pip install -r requirements.txt
python plot_vortex_street.py
```

---

## 🛠️ Tech Stack
- **Computational Core:** Rust 1.75+, `serde`, zero-allocation memory arrays
- **Fluid Mechanics:** Lattice Boltzmann Method (LBM D2Q9), BGK Collision, Momentum Exchange
- **Visualization:** Python 3.10+, `numpy`, `matplotlib`

---

## 👨‍💻 Author
**Ardavan Ghal-Eh**  
Mechanical Engineering Student, Sharif University of Technology  
*Focus: Computational Fluid Dynamics (CFD), Scientific Computing & High-Performance Systems*
