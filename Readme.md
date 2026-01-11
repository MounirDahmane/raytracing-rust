````markdown
# Ray Tracing: The Next Week — Rust Implementation

This branch implements *Ray Tracing: The Next Week* (Book 2, v4.0.2 — 2025-04-25) in Rust, including motion blur, BVH, texture mapping, Perlin noise, quadrilaterals, lights, instances, volumes, and new primitives. Outputs `.ppm` frames for viewing or video encoding.

---

## Requirements

- Rust toolchain (`rustup.rs`)  
- `rayon` crate (for parallel rendering)  
- FFmpeg (optional, for video encoding)

---

## Build

```bash
cargo build --release
````

---

## Usage — Single Image

```bash
cargo run --release > ./path/output_image.ppm
```

---

## Usage — Animation

Example animation `render_bouncing_spheres_animation()` :

* Precomputes a damped bounce schedule for smooth timing.
* Prepares static scene data (materials, centers) once and reuses it.
* For each frame:

  * Computes vertical offset from bounce schedule.
  * Rebuilds scene with ground, moving spheres offset by vertical position, and static large spheres.
  * Builds BVH and calls:

  ```rust
  cam.render_to_file(&new_world, frame as i32, "./img/image");
  ```
* Files saved as `./img/image_0.ppm`, `image_1.ppm`, ..., `image_599.ppm` (600 frames in my case).

---

## Create Video (FFmpeg)

Basic command used:

```bash
ffmpeg -i ./img/image_%d.ppm ./output.mp4
```

* Converts 600 raw PPM frames (~2.4 GB) to ~2.0 MB MP4, demonstrating powerful compression.

Recommended command with frame rate and quality control:

```bash
ffmpeg -framerate 30 -i ./img/image_%d.ppm -c:v libx264 -crf 18 -pix_fmt yuv420p output.mp4
```

* `-framerate` matches render FPS.
* `-crf` controls quality (lower = better).
* `-pix_fmt yuv420p` ensures player compatibility.
* Use zero-padded pattern like `image_%04d.ppm` if needed.

---

## Additional Features

Includes Book 2 features plus new primitives:

* **Triangle, Ellipse, Annulus**: new geometric shapes.
* **TextureMask**: alpha-style texture masking.
* **Mandelbrot**: fractal procedural texture/primitive.

Use by constructing and adding these to scenes as needed.

---

## Performance Notes

* CPU-bound renderer; more samples and depth → better quality but slower.
* Parallel row rendering with Rayon speeds up rendering on multi-core CPUs.
* `render_to_file` efficiently reuses buffers and materials.
* Exclude generated frames and `target/` from version control.

---

## Citation

**Markdown:**

[*Ray Tracing: The Next Week*](https://raytracing.github.io/books/RayTracingTheNextWeek.html) — Peter Shirley, Trevor David Black, Steve Hollasch (v4.0.2, 2025-04-25)

**BibTeX:**

```bibtex
@misc{Shirley2025RTW2,
  title = {Ray Tracing: The Next Week},
  author = {Peter Shirley and Trevor David Black and Steve Hollasch},
  year = {2025},
  month = {April},
  note = {\texttt{https://raytracing.github.io/books/RayTracingTheNextWeek.html}},
  url = {https://raytracing.github.io/books/RayTracingTheNextWeek.html}
}
```

---
