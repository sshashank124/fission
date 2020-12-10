Physically-Based Renderer in Rust

Uses [graphite](https://github.com/sshashank124/graphite) as the base math, vector and geometry library

Features:
- Tracers (Path, Ambient Occlusion, Direct Illumination, Normals, Silhouette)
- Shapes (Mesh/Triangle, Sphere)
- Acceleration Data Structures (BVH)
- BSDFs (Dielectric, Diffuse, Microfacet, Mirror)
- Cameras (Perspective)
- Integrators (Sampler Integrator)
- Lights (Point, Area, Infinite)
- Samplers (Discrete PDF, Independent [PCG64], Sobol LDS)
- Textures (Constant, Checkerboard, Gradient, Grid)
- Loaders (YAML scene config, Wavefront OBJ)
- OpenEXR Image output
