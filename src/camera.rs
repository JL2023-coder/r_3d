use nalgebra::{matrix, Const, Matrix4, OPoint, Vector3};
use speedy2d::dimen::Vector2;


// Public struct camera
// A camera contains the necessary information to create a view matrix
// This includes:
// - eye positon, where the object sees from
// - target position, where the object is looking
// - up direction, tells the object which way is up in the world
pub struct Camera {
    pub eye_position: OPoint<f32, Const<3>>,
    pub target_position: OPoint<f32, Const<3>>,
    pub up_direction: Vector3<f32>,
}

impl Camera {
    // Creates new camera
    pub fn new(
        eye_position: OPoint<f32, Const<3>>, 
        target_position: OPoint<f32, Const<3>>, 
        up_direction: Vector3<f32>
    ) -> Camera {
        let eye_positon = eye_position;
        let target_position = target_position;
        let up_direction = up_direction;

        return Camera {
            eye_position,
            target_position,
            up_direction,
        }
    }
    // Takes itself
    // Gets the view matrix 
    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.eye_position, &self.target_position, &self.up_direction)
    }

    pub fn turn_right(&mut self, x: f32) {
        // Projection matrix for rotating around y-axis
        let proj_y = matrix![
        x.cos(), 0.0, -x.sin();
        0.0, 1.0, 0.0;
        x.sin(), 0.0, x.cos()
        ];

        // Current target_position
        // Convert to vector for multiplication, using coords
        let curr_target_vector = self.target_position.coords;
        // Normalize so vector length is 1
        let rotated_target_vector = (proj_y * curr_target_vector).normalize();
        self.target_position = OPoint::from(rotated_target_vector);
    }
}