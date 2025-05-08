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
        return Camera {
            eye_position,
            target_position,
            up_direction,
        }
    }
    // Takes itself
    // Gets the view matrix 
    // The view matrix is used to calculate drawing positon for other objects
    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.eye_position, &self.target_position, &self.up_direction)
    }

    // Rotates the camera around the y-axis
    pub fn rotate_around_y(&mut self, degrees: f32) {
        // Projection matrix for rotating around y-axis
        let proj_y = matrix![
        degrees.cos(), 0.0, -degrees.sin();
        0.0, 1.0, 0.0;
        degrees.sin(), 0.0, degrees.cos()
        ];

        // Current target_position
        // We substract eye postion
        // We only want to rotate our eye position not space origin
        let curr_target_from_eye = self.target_position.coords - self.eye_position.coords;
        // Rotate the vector
        let rotated_target_vector = proj_y * curr_target_from_eye;
        // Add eye postion to rotated vector, since we have preoviously substracted it
        let new_target_vector = rotated_target_vector + self.eye_position.coords;
        // Update self.target_position
        self.target_position = OPoint::from(new_target_vector);
    }

    // Moves camera eye postion
    // Positive moves forward, negative value backwards
    pub fn dolly(&mut self, distance: f32) {
        let target_postion_normalized = &self.target_position.coords.normalize();
        self.eye_position += Vector3::new(0.0, 0.0, 1.0) * distance;

    }
}

