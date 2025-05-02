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
        return Ok(MyWindowHandler {
            self.rect: Rectangle::new(200, 200, 200, point![600.0, 600.0, 0.0])
        });
    }  
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::BLACK);

        

        draw_rectangle(rect, graphics);

        helper.request_redraw();
    }
}

