
use nalgebra::{Const, OPoint, point, Point3};

pub struct Rectangle {
    // A rectangle consists of 8 points
    // You can also access widthm height and length
    pub w: f32, pub h: f32, pub l: f32,
    pub center: OPoint<f32, Const<3>>,
    pub points: [OPoint<f32, Const<3>>; 8],
}


// Constructor
impl Rectangle {
    // new function for rectangle
    pub fn new(w: f32, h: f32, l: f32, starting_corner: Point3<f32>) -> Rectangle {
        let w: f32 = w;
        let h: f32 = h;
        let l: f32 = l;
        let center: OPoint<f32, Const<3>> = point![starting_corner.x + w / 2.0, starting_corner.y + h / 2.0, starting_corner.z + l / 2.0]; 
        let points: [OPoint<f32, Const<3>>; 8] = create_3d_rectangle(w, h, l, starting_corner);

        return Rectangle {
            w, h, l, center, points,
        };
    }
}
/// Creates a rectangle from width, height, and length
/// Define starting corners by vector3(x,y,x): coordinates should be f32
fn create_3d_rectangle(w: f32, h: f32, l: f32, starting_corner: Point3<f32>) -> [OPoint<f32, Const<3>>; 8] {
    // Define the starting corner coordinates
    let x0 = starting_corner.x;
    let y0 = starting_corner.y;
    let z0 = starting_corner.z; 

    // Calculate the 8 vertices using the starting corner and dimensions
    let points: [OPoint<f32, Const<3>>; 8] = [
        point![x0,     y0,     z0],     
        point![x0 + w, y0,     z0],     
        point![x0 + w, y0 + h, z0],     
        point![x0,     y0 + h, z0],     

        point![x0,     y0,     z0 + l], 
        point![x0 + w, y0,     z0 + l], 
        point![x0 + w, y0 + h, z0 + l], 
        point![x0,     y0 + h, z0 + l]  
    ];

    return points;
}




