use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::color::Color;
use std::error::Error;
use std::f32::consts::FRAC_PI_4;
use nalgebra::{matrix, point, vector, Const, Matrix4, OPoint, Vector3};
use crate::camera::Camera;
use crate::shapes::*;
use crate::transformations::*;
use crate::drawing::*;

const FOV: f32 = FRAC_PI_4;
const FAR: f32 = 5000.0;
const NEAR: f32 = 100.0;

pub struct MyWindowHandler {
    rects: Vec<Rectangle>,
    camera: Camera,
}


impl MyWindowHandler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Rectangles
        let rect1 = Rectangle::new(200.0, 200.0, 200.0, point![0.0, 0.0, 1.0]);
        let rect2 = Rectangle::new(200.0, 200.0, 200.0, point![0.0, 0.0, 500.0]);
        let rect3 = Rectangle::new(200.0, 200.0, 200.0, point![-300.0, 0.0, 500.0]);
        let rect4 = Rectangle::new(200.0, 200.0, 200.0, point![-500.0, 0.0, 500.0]);
        let rect5 = Rectangle::new(200.0, 200.0, 200.0, point![1000.0, 0.0, 500.0]);
        let rect6 = Rectangle::new(200.0, 200.0, 200.0, point![1400.0, 0.0, 500.0]);
        let mut rects = Vec::new();

        rects.push(rect1);
        rects.push(rect2);
        rects.push(rect3);
        rects.push(rect4);
        rects.push(rect5);
        rects.push(rect6);

        // Camera
        let eye_position = point![0.0, 0.0, 0.0,];
        let target_position = point![0.0, 0.0, 1.0];
        let up_direction = vector![0.0, 1.0, 0.0];
        let camera = Camera::new(eye_position, target_position, up_direction);
        return Ok(MyWindowHandler {
            rects,
            camera,
        });
    }  
}

impl WindowHandler for MyWindowHandler {
    fn on_keyboard_char(
            &mut self,
            helper: &mut WindowHelper<()>,
            unicode_codepoint: char
        ) {
        match unicode_codepoint {
            'd' => self.camera.turn_right(0.1),
            'a' => self.camera.turn_right(-0.1),
            _ => println!("Not implemented"),
            
        }
    }
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        // ------------------ TEST ------------------------ //

        // --------------- Game Updates ------------------ // 

        for r in self.rects.iter_mut() {
            rotate_x(r, 0.01);
            rotate_y(r, 0.01);
            rotate_z(r, 0.01);
        }
        

        // ---------------- View Matrix ------------------- //

        let view_matrix = self.camera.get_view_matrix();

        // ---------------- Drawing ---------------------- //

        graphics.clear_screen(Color::BLACK);

        for r in self.rects.iter() {
            let model_matrix = Matrix4::new_translation(&r.center.coords);
            let transformed_points = proj_perspective(
                r, // Pass the current rectangle
                FOV,
                4.0/3.0, // Use dynamic aspect ratio
                NEAR,
                FAR,
                model_matrix,
                view_matrix
            );
            draw_rectangle(transformed_points, graphics);
        }

        helper.request_redraw();
    }
}

