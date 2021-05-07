Physically-Based Renderer in Rust

Uses [graphite](https://github.com/sshashank124/graphite) as the base math, vector and geometry library

Uses [objloader](https://github.com/sshashank124/objloader) as the Wavefront OBJ-loading library

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
- YAML scene config loader (automatic deserialization)
- OpenEXR Image output
- Render State serializing-to and deserializing-from disk

Front-Ends:
- GUI Renderer
- CLI Renderer

Sample Renders:

![Diamond Bowl and Glass with multiple soft lighting](examples/diamond_bowl_and_glass.png)
