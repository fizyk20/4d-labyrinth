4D Labyrinth
============

The 4D Labyrinth is a simple game illustrating the 4-dimensional geometry.

The player (represented by a yellow cube, which is actually a tesseract - a 4-dimensional analog of a 
cube) has to travel through a labyrinth of tunnels laid out in 4 dimensions and find the target, which 
is represented by a blue tesseract.

What the player sees is actually just a 3D slice of the 4D hyperspace. It's analogous to only seeing a
2-dimensional slice of a 3D space, in which, for example, a cube could be seen as a square, a rectangle,
a triangle, or even a hexagon. Similarly, a 3D slice of a tesseract can be a cube, a cuboid or some
other weird shape. The visible slice can be rotated, giving the player access to the whole 4D space.

Controls
--------

The player can move using the following keys:

* W/S/A/D - move forwards/backwards/left/right
* Q/E - move up/down
* T/G - pitch down/up
* F/H - roll left/right
* R/Y - yaw left/right
* U/J/C/V/B/N - rotate the visible slice of the hyperspace

Notes
-----

* The program uses OpenGL via the awesome [Glium](https://github.com/tomaka/glium) crate
* Rust nightly is required for compilation due to some usage of the `impl Trait` feature
