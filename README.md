<div align="center">

<br>

# Mu-Sim

**Vehicle Dynamics Simulation Engine**

<br>

<a href="https://github.com/Muran-prog/mu-sim/actions"><img src="https://img.shields.io/github/actions/workflow/status/Muran-prog/mu-sim/ci.yml?branch=main&style=for-the-badge&logo=github&logoColor=white&label=CI&color=161B22" alt="CI"></a>
&nbsp;&nbsp;
<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/RUST-1.75+-DE5833?style=for-the-badge&logo=rust&logoColor=white" alt="Rust"></a>
&nbsp;&nbsp;
<a href="https://docs.rust-embedded.org/book/"><img src="https://img.shields.io/badge/no__std-READY-4EAA25?style=for-the-badge" alt="no_std"></a>
&nbsp;&nbsp;
<a href="LICENSE"><img src="https://img.shields.io/badge/LICENSE-MIT-blue?style=for-the-badge" alt="MIT"></a>

<br>
<br>

*Real-time physics simulation with deterministic behavior,*<br>
*zero runtime allocations, and embedded systems compatibility.*

<br>

<a href="#quick-start">Quick Start</a>&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;<a href="#architecture">Architecture</a>&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;<a href="#crates">API</a>&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;<a href="#performance">Performance</a>

<br>

</div>

<br>

<br>

## Why Mu-Sim?

<table>
<tr>
<td width="50%" valign="top">

### Type-Safe Physics

Compile-time unit checking eliminates the classic
*meters vs feet* disasters. Physical quantities carry
their units in the type system — wrong conversions
simply won't compile.

```rust
let distance = Meters(100.0);
let time = Seconds(10.0);
let velocity = distance / time; // MetersPerSecond
```

</td>
<td width="50%" valign="top">

### Real-Time Ready

Designed for deterministic simulation loops at
fixed timesteps. No heap allocations in hot paths.
Predictable, consistent timing for
hardware-in-the-loop testing.

```rust
// Fixed 1kHz physics loop
loop {
    world.step(Seconds(0.001));
}
```

</td>
</tr>
<tr>
<td width="50%" valign="top">

### High-Performance LUTs

Engine maps, tire curves, aerodynamic tables —
all use O(log n) binary search with linear,
bilinear, and trilinear interpolation.
Cache-friendly, branch-minimized.

```rust
let torque = engine_map.lookup(rpm);
let grip = tire_model.lookup(slip, load);
```

</td>
<td width="50%" valign="top">

### Embedded Compatible

Full `no_std` support from day one.
Run on desktop simulators during development,
deploy to embedded ECUs in production.
Same codebase, same behavior.

```rust
#![no_std]
use vd_math::Lut1D;
```

</td>
</tr>
</table>

<br>

## Architecture

```
mu-sim/
│
├── vd_types          Type-safe SI units
│   ├── Meters, Seconds, Kilograms...
│   ├── MetersPerSecond, Newtons, Watts...
│   └── Compile-time dimensional analysis
│
└── vd_math           Mathematical primitives  
    ├── Vec3, Mat3, Quat (via nalgebra)
    └── Lut1D, Lut2D, Lut3D interpolators
```

<br>

## Crates

<details open>
<summary><b>vd_types</b> — Physical Units</summary>

<br>

Prevent unit confusion at compile time with zero runtime cost.

```rust
use vd_types::{Meters, Seconds, MetersPerSecond, Newtons, Kilograms};

let distance = Meters(100.0);
let time = Seconds(10.0);
let velocity: MetersPerSecond = distance / time;  // Type-checked

let mass = Kilograms(1200.0);
let acceleration = velocity / time;
let force: Newtons = mass * acceleration;         // F = ma, verified by compiler
```

</details>

<details open>
<summary><b>vd_math</b> — Linear Algebra & Interpolation</summary>

<br>

Built on `nalgebra` with ergonomic wrappers and high-performance lookup tables.

```rust
use vd_math::{Vec3, Lut1D, Lut2D};

// Vectors and rotations
let velocity = Vec3::new(10.0, 0.0, 0.0);
let gravity = Vec3::new(0.0, 0.0, -9.81);

// Engine torque curve: RPM -> Nm
let engine = Lut1D::new(
    vec![1000.0, 3000.0, 5000.0, 7000.0],
    vec![ 180.0,  320.0,  290.0,  220.0],
)?;
let torque = engine.lookup(4200.0);  // Interpolated

// Tire grip surface: slip_angle x slip_ratio -> mu
let tire = Lut2D::new(slip_angles, slip_ratios, grip_coefficients)?;
let mu = tire.lookup(8.5, 0.12);     // Bilinear interpolation
```

</details>

<br>

## Quick Start

```bash
# Clone
git clone https://github.com/user/mu-sim && cd mu-sim

# Build
cargo build --release

# Test
cargo test --all

# Lint
cargo clippy --all
```

<br>

## Performance

|:--|:--|
| **Zero-cost units** | `#[repr(transparent)]` newtypes — same assembly as raw `f64` |
| **O(log n) lookups** | Binary search with branchless interpolation |
| **Inline everything** | `#[inline]` on all hot paths |
| **Aggressive LTO** | `lto = "fat"`, single codegen unit, stripped symbols |

<br>

## CI

|:--:|:--:|:--:|
| <img src="https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black" alt="Linux"> | <img src="https://img.shields.io/badge/macOS-000000?style=for-the-badge&logo=apple&logoColor=white" alt="macOS"> | <img src="https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white" alt="Windows"> |

<sub>MSRV 1.75 · no_std verified on `thumbv7em-none-eabihf`</sub>

<br>

---

<div align="center">

<br>

**Built for precision. Engineered for speed.**

<sub>MIT License · Copyright 2026 Muran-prog</sub>

<br>

</div>
