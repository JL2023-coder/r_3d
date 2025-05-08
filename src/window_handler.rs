use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::color::Color;
use std::error::Error;
use std::f32::consts::FRAC_PI_8;
use nalgebra::{matrix, point, vector, Const, Matrix4, OPoint, Vector3};
use crate::camera::Camera;
use crate::shapes::*;
use crate::transformations::*;
use crate::drawing::*;

const FOV: f32 = FRAC_PI_8;
const FAR: f32 = 20000.0;
const NEAR: f32 = 0.001;

pub struct MyWindowHandler {
    rects: Vec<Rectangle>,
    camera: Camera,
}


impl MyWindowHandler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut rects = Vec::new();

        // Rectangles
        // Rectangles z
        for i in 0..100 {
            for j in 0..20 {
                let rect = Rectangle::new(50.0, 50.0, 50.0, point![j as f32  * 25.0, 0.0, i as f32 * 25.0]);
                rects.push(rect);
            } 
        }


        // Camera
        let eye_position = point![0.0, 500.0, 0.0,];
        let target_position = point![0.0, 500.0, 1.0];
        let up_direction = vector![0.0, 1.0, 0.0];
        let camera = Camera::new(eye_position, target_position, up_direction);
        return Ok(MyWindowHandler {
            rects,
            camera,
        });
    }  
}

impl WindowHandler for MyWindowHandler {
    // Handles keyboard input
    fn on_keyboard_char(
            &mut self,
            helper: &mut WindowHelper<()>,
            unicode_codepoint: char
        ) {
        match unicode_codepoint {
            // Updates camera target direction
            // The fn takes in degrees to be turned
            // Higher value, you turn more
            'd' => self.camera.rotate_around_y(0.05),
            // On 'a' turn left
            'a' => self.camera.rotate_around_y(-0.05),

            // Updates camera eye position
            'w' => self.camera.dolly(10.0),
            _ => println!("Not implemented"),
            
        }
    }
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        // ------------------ TEST ------------------------ //

        // --------------- Game Updates ------------------ // 

        // Remove '//' to make rectangle rotate
        for r in self.rects.iter_mut() {
            //rotate_x(r, 0.01);
            //rotate_y(r, 0.01);
            //rotate_z(r, 0.01);
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
            draw_rectangle_outline(transformed_points, graphics);
        }

        helper.request_redraw();
    }
}

