use std::f32::NAN;

use nalgebra::{coordinates::X, Matrix4, matrix, point, Const, OPoint, Point3, Vector3};

use crate::{main, shapes::Rectangle};

/// 3d to 2d projection
pub fn proj_2d(rect: &Rectangle) -> [OPoint<f32, Const<3>>; 8] {
    let mut points= rect.points;
    let proj_2d = matrix![
        1.0, 0.0, 0.0;
        0.0, 1.0, 0.0;
        0.0, 0.0, 0.0
    ];

    for p in points.iter_mut() {
        *p = proj_2d * *p;
    }
    
    points
}

// Scaling projection
pub fn proj_perspective(rect: &Rectangle, 
    fov: f32,
    asp_ratio: f32, 
    far: f32, 
    near: f32,
    model_matrix: Matrix4<f32>,
    view_matrix: Matrix4<f32>
) -> [OPoint<f32, Const<3>>; 8] {
    // Temp Vars
    // Test drawing width
    let screen_width = 1400.0;
    // Test drawing height
    let screen_height = 1100.0;

    let proj_perspective_matrix = matrix![
        1.0 / ((fov / 2.0).tan() * asp_ratio), 0.0, 0.0, 0.0;
        0.0, 1.0 / ((fov / 2.0).tan()), 0.0, 0.0;
        0.0, 0.0, -((far + near) / ( far - near)), -((2.0 * far * near) / (far - near));
        0.0, 0.0, -1.0, 0.0; 
    ];
    
    // Calculation for MVP matrix
    // View Proj matrix
    let view_projection_matrix = proj_perspective_matrix * view_matrix;
    // MVP matrix
    let mvp_matrix = view_projection_matrix * model_matrix;

    let mut points = rect.points;

    for p in points.iter_mut() {
        let mut my_p = p.to_homogeneous();
        my_p = mvp_matrix * my_p;

        let w = my_p.w;

        // Check if point should be seen/ valid point
        // A point is valid as long as it is in front of camera/ view_target
        if w > 0.0 && 
        my_p.x >= -w && my_p.x <= w && 
        my_p.y >= -w && my_p.y <= w &&
        my_p.z >= -w && my_p.z <= w {

            let x_ndc = my_p.x / w;
            let y_ndc = my_p.y / w;
            let z_ndc = my_p.z / w;

            // Final x coord for drawing
            let x_screen = ((x_ndc + 1.0) / 2.0) * screen_width;
            // Final y coord for drawing
            let y_screen = ((1.0 - y_ndc) / 2.0) * screen_height;

            *p = point![x_screen, y_screen, z_ndc];
        }
        else {
            *p = point![NAN, NAN, NAN];
        }
    }
    points
}

/// --------------- Rotation methods ------------------
pub fn rotate_x(rect: &mut Rectangle, x: f32) {
    let center = rect.center; 
    let proj_x = matrix![
        1.0, 0.0, 0.0;
        0.0, x.cos(), -x.sin();
        0.0, x.sin(), x.cos()
    ];

    for p in rect.points.iter_mut() {
        let relative_vec: Vector3<f32> = *p - center;
        let rotated_vec: Vector3<f32> = proj_x * relative_vec;
        let p_new: Point3<f32> = center + rotated_vec;
        *p = p_new;
    }
}

pub fn rotate_y(rect: &mut Rectangle, x: f32) {
    let center = rect.center;
    let proj_y = matrix![
        x.cos(), 0.0, -x.sin();
        0.0, 1.0, 0.0;
        x.sin(), 0.0, x.cos()
    ];

    for p in rect.points.iter_mut() {
        let relative_vec: Vector3<f32> = *p - center;
        let rotated_vec: Vector3<f32> = proj_y * relative_vec;
        let p_new: Point3<f32> = center + rotated_vec;
        *p = p_new;
    }
}

pub fn rotate_z(rect: &mut Rectangle, x: f32) {
    let center = rect.center;
    let proj_z = matrix![
        x.cos(), -x.sin(), 0.0;
        x.sin(), x.cos(), 0.0;
        0.0, 0.0, 1.0
    ];

    for p in rect.points.iter_mut() {
        let relative_vec: Vector3<f32> = *p - center;
        let rotated_vec: Vector3<f32> = proj_z * relative_vec;
        let p_new: Point3<f32> = center + rotated_vec;
        *p = p_new;
    }
}
