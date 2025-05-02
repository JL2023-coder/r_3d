use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::color::Color;
use std::error::Error;
use std::primitive::f32;
use nalgebra:: point;
use crate::shapes::*;
use crate::transformations::*;
use crate::drawing::*;


pub struct MyWindowHandler {
    // Degrees rotated
    x: f32,
}


impl MyWindowHandler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let x: f32 = 0.0;
        return Ok(MyWindowHandler {
            x,
        });
    }  
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::BLACK);

        let w = 200.0;
        let h = 200.0;
        let l = 200.0;
        // Starting point should be the starting corners
        let starting_point = point![600.0, 600.0, 0.0];
        let mut rect = Rectangle::new(w, h, l, starting_point);

        rotate_x(&mut rect, self.x);
        rotate_y(&mut rect, self.x);
        rotate_z(&mut rect, self.x);

        proj_2d(&mut rect);

        draw_rectangle(rect, graphics);

        self.x = 2.01;

        

        helper.request_redraw();
    }
}

