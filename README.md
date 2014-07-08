Grunge
======

Grunge is a pseudo-random coherent noise library in the spirit of libnoise, ACL,
and the Coherent Noise Library. Unlike earlier libraries, it uses the [Simplex Noise]
(http://en.wikipedia.org/wiki/Simplex_noise) algorithm, and should have better
performance as a result. Hopefully, it will one day contain more features as well.

The API should be familiar to users of libnoise and the Coherent Noise Library, but is
subject to change without notice at this point in the project -- which is currently in
the very early stages of development.

Planned Features
----------------

* Simplex noise primitives in 2D, 3D, and 4D.
* "Fractal" noise types: pink noise, billow niose, and ridged multifractal noise.
* Voronoi noise types
* Geometric noise types: sphere, cylinder, aribtrary functions.
* Modifier types: add, subtract, multiply, turbulence, and so on.
* Helpful utilities for generating images, textures, and so on.
* Paralellization when it improves performance.
* Many examples.
* Language bindings?
