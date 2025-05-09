# Someday a 3d engine

## About

Me trying to create a 3d engine in Rust, with absolute no clue how:) We learn by doing!

## Features Implemented So Far

* **Basic 3D Rendering:** Renders simple rectangular prisms (cuboids) in a 3D space.
* **Object Transformation:** Supports rotating individual objects around their local axes.
* **Perspective Projection:** Renders the scene with a perspective view, creating the illusion of depth.
* **Basic Camera Movement:** Allows moving the camera forward along a fixed direction.
* **Camera Point-of-View Control:** Enables rotating the camera's view around the vertical (y) axis (turning left/right).

## Planned Features (To Be Implemented)

### Camera

* Implement camera movement relative to its viewing direction (moving forward towards the target direction).
* Enable full 6-degree-of-freedom camera movement (forward/backward, strafe left/right, up/down).
* Explore and implement different camera perspectives or modes (e.g., third-person, top-down).
* Implement a type of light projection, like shadow mapping.

### Object Representation

* Add support for rendering other shapes(rectangles, circles).
* Ability to render general 3D objects.

### World Interaction

* Simple **collision** system.
* Player gravity.
* Ability to add and remove objects in runtime.
* Moving objects based on player input during runtime(example: moving with **collision**).

### Future Possibilities

* Use the engine to create simple games(basic minecraft clone, FPS shooter).
* Visualize 3D rotations, projections and shadow mapping.
