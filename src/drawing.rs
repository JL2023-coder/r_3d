use nalgebra::{Const, OPoint, Point3};
use speedy2d::Graphics2D;
use speedy2d::color::Color;

use crate::shapes::Rectangle;
use crate::transformations::*;


pub fn draw_rectangle(points: [OPoint<f32, Const<3>>; 8] , g: &mut Graphics2D) {
    // Draws the first face
    for i in 0..points.len() / 2 { // Assumes 8 points, so 0..4
        let p_2d = (points[i].x, points[i].y);
        let p_2d_next;
        // if end of points, make line to startpoint
        if i == points.len() / 2 - 1 { // i == 3
            p_2d_next = (points[0].x, points[0].y);
        }
        else {
            p_2d_next = (points[i + 1].x, points[i + 1].y);
        }
        g.draw_line(p_2d, p_2d_next, 2.0, Color::WHITE);
    }

    // Draws the second face
    for i in points.len() / 2..points.len() { // Assumes 8 points, so 4..8
        let p_2d = (points[i].x, points[i].y);
        let p_2d_next;
        // if end of points, make line to startpoint
        if i == points.len() - 1 { // i == 7
            p_2d_next = (points[points.len() / 2].x, points[points.len() / 2].y); // Connects point 7 to point 4
        }
        else {
            p_2d_next = (points[i + 1].x, points[i + 1].y);
        }
        g.draw_line(p_2d, p_2d_next, 2.0, Color::WHITE);
    }

    // Draws the connecting edges between faces
    for i in 0..points.len() / 2 { // Assumes 8 points, so 0..4
        let p_2d = (points[i].x, points[i].y);
        let p_2d_next;
        // if end of points, make line to startpoint
        if i == points.len() - 1 { // This condition will never be true in this loop (i goes up to 3)
            p_2d_next = (points[points.len() / 2].x, points[points.len() / 2].y);
        }
        else {
            p_2d_next = (points[i + points.len() / 2].x, points[i + points.len() / 2].y); // Connects point i to point i + 4
        }
        g.draw_line(p_2d, p_2d_next, 2.0, Color::WHITE);
    }
}