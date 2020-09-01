![Rusty Raytracer](images/logo.png)

---

![Rust](https://github.com/AnuarTB/rusty-raytracer/workflows/Rust/badge.svg)

This repo is just an experimental project that I have been doing, in order to learn Rust.

## Features

The following program has support of:

- Diffuse and specular materials
- Lights (3 types)
- Reflections
- Shadows
- Camera and viewport control (FOV, orientation)
- Antialising (by multisampling)
- Simple .obj file reading
- AABB optimization
- Multithreading
- Builder pattern for constructing objects

### To do

- [ ] Integration with WebAssembly
  - [ ] Interaction with objects
- [ ] Gamma correction
- [ ] Refraction
- [ ] Textures

## Screenshots

- 5 samples per pixel
- 6320 triangles

![screenshot](images/render.png)

## Related

It was in turn inspired by several projects listed below:

- [Computer Graphics from scratch (JS)](https://www.gabrielgambetta.com/computer-graphics-from-scratch/introduction.html)
- [Understandable RayTracing in 256 lines of bare C++](https://github.com/ssloy/tinyraytracer/wiki)
- [Ray Tracing in One Weekend (C++)](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
- [Writing raytracer in Rust series](https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/)

