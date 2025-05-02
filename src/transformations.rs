use nalgebra::{matrix, Const, OPoint, Point3, Vector3, point};

use crate::shapes::Rectangle;
/// 3d to 2d projection
pub fn proj_2d(rect: &mut Rectangle) {
    let proj_2d = matrix![
        1.0, 0.0, 0.0;
        0.0, 1.0, 0.0;
        0.0, 0.0, 0.0
    ];

    for p in rect.points.iter_mut() {
        *p = proj_2d * *p;
    }
}

/// --------------- Rotation methods ------------------
pub fn rotate_x(rect: &mut Rectangle) {
    let x = rect.degrees;
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

pub fn rotate_y(rect: &mut Rectangle) {
    let x = rect.degrees;
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

pub fn rotate_z(rect: &mut Rectangle) {
    let x = rect.degrees;
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
