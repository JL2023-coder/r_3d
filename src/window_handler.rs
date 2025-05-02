use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::color::Color;
use std::error::Error;
use nalgebra::point;
use crate::shapes::*;
use crate::transformations::*;
use crate::drawing::*;


pub struct MyWindowHandler {
    rect: Rectangle, 
}


impl MyWindowHandler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let rect = Rectangle::new(200.0, 200.0, 200.0, point![600.0, 600.0, 0.0]);
        return Ok(MyWindowHandler {
            rect,
        });
    }  
}

impl WindowHandler for MyWindowHandler {
    fn on_resize(&mut self, helper: &mut WindowHelper<()>, size_pixels: speedy2d::dimen::UVec2) {
        rotate_x(&mut self.rect);
        rotate_y(&mut self.rect);
        rotate_z(&mut self.rect);
        self.rect.degrees += 5.0;
    }
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::BLACK);

        draw_rectangle(&self.rect, graphics);
        println!("degrees {}",self.rect.degrees);

        helper.request_redraw();
    }
}

