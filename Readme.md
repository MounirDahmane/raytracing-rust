# raytracing-rust

Rust implementation of a **Ray Tracer**. The `main` branch contains the complete, up-to-date implementation. Two additional branches, `book1-final` and `book2-final`, store the finished results and README-notes for Book 1 and Book 2 respectively.

---

# Project summary

This repository implements a Monte-Carlo CPU path tracer in Rust, following Peter Shirley’s series and progressively adding features across the three books:

* **Book 1** (branch: `book1-final`) — core path tracer, basic materials, spheres, camera, image output.
* **Book 2** (branch: `book2-final`) — importance sampling, BVH, textures, additional materials, animation workflow and an FFmpeg-based pipeline for turning raw PPM frames into video.
* **Book 3** (main branch) — complete feature set from *The Rest of Your Life*.

---

# Key features (repository-wide)

* Monte-Carlo path tracing (multiple samples-per-pixel, recursion depth)
* BVH acceleration structure
* Primitives: sphere, quad, triangle, ellipse, annulus, and others
* Thread-safe design: `Arc<dyn ... + Send + Sync>` for shared scene graph
* Parallel rendering using `rayon` (data-parallel pixel filling)
* PPM frame output (suitable for video encoding)
* Example animation workflows (see `book2-final` for FFmpeg pipeline)

---

# Requirements

* Rust toolchain (install via `rustup`)
* `cargo` (comes with Rust toolchain)
* Optional: `ffmpeg` (for converting frames to video — animation/encoding workflow described in `book2-final`)

Dependencies are declared in `Cargo.toml` (examples used in the project include `rayon`, `rand` with `small_rng`, and `indicatif`).

---

# Build

```bash
git clone <repo-url> raytracing-rust
cd raytracing-rust
cargo build --release
```

---

# Usage

### Single image

Run the binary and redirect stdout to a `.ppm` file:

```bash
cargo run --release > ./img/output_image.ppm
```

### Animation (high level)

**FFmpeg note:** The Book 2 branch (`book2-final`) contains the documented FFmpeg commands and examples used to convert PPM frames into MP4 (example: `ffmpeg -framerate 30 -i ./img/image_%d.ppm -c:v libx264 -crf 18 -pix_fmt yuv420p output.mp4`).

---

# Concurrency & safety (brief)

* Shared scene graph objects are stored behind `Arc<dyn Trait + Send + Sync>`.
* Traits used as trait objects carry `Send + Sync` bounds to ensure thread safety.
* Rendering uses `rayon::par_iter_mut()` to fill a single pixel buffer; each pixel is written exactly once, so no locks are required for pixel writes.
* Result: safe, scalable multi-core rendering (observed speedup ≈ **2.6×** on typical test scenes vs single-threaded on the author’s machine).

---

# Performance notes

* Renderer is CPU-bound; quality (samples, depth) trades off directly with time.
* BVH reduces intersection costs; instancing and reusing buffers/materials improves throughput for animations.

---

# Citation

**Ray Tracing: The Rest of Your Life (Book 3) — BibTeX**

```bibtex
@misc{Shirley2025,
  title = {Ray Tracing: The Rest of Your Life},
  author = {Peter Shirley and Trevor David Black and Steve Hollasch},
  year = {2025},
  month = {April},
  note = {\texttt{https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html}},
  url = {https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html}
}
```

(Branch READMEs `book1-final` and `book2-final` contain additional, branch-specific notes and citations.)

---

# Contributing & license

Contributions welcome (PRs, bug reports, performance suggestions). Please run `cargo fmt` and `cargo clippy` prior to PRs.

No license file included by default. Consider adding one (MIT or Apache-2.0) if you plan to publish or share widely.

---