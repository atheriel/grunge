Grunge
======

Grunge is a pseudo-random coherent noise library in the spirit of libnoise, ACL,
and the Coherent Noise Library. Unlike earlier libraries, it uses the [Simplex Noise]
(http://en.wikipedia.org/wiki/Simplex_noise) algorithm, and should have better
performance as a result. Hopefully, it will one day contain more features as well.

The API should be familiar to users of libnoise and the Coherent Noise Library, but is
subject to change without notice at this point in the project -- which is currently in
the very early stages of development.

Feature Wishlist
----------------

- [ ] Simplex noise primitives
    - [x] 2D
    - [ ] 3D
    - [ ] 4D

- [ ] Factal noise
    - [x] Pink noise (also called Perlin noise)
    - [x] Billow noise
    - [ ] Ridged Multifractal noise
