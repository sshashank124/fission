Physically-Based Renderer in Rust

Features:
- Core (optimized 2D and 3D base vectors, Discrete PDF, numerics ADTs)
- Geometry (Point, Vector, Normal, Ray, Transforms)
- Additional Geometry (Bounds, Bounding-Boxes, Frames, Intersections, Warps)
- Tracers (Path, Ambient Occlusion, Direct Illumination, Normals, Silhouette)
- Shapes (Mesh/Triangle, Sphere)
- Acceleration Data Structures (BVH)
- BSDFs (Dielectric, Diffuse, Microfacet, Mirror)
- Cameras (Perspective)
- Integrators (Sampler Integrator)
- Lights (Point, Area, Infinite)
- Samplers (Independent [PCG64], Sobol LDS)
- Textures (Constant, Checkerboard, Gradient, Grid)
- Loaders (YAML scene config, Wavefront OBJ)
- OpenEXR Image output
