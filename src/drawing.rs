use nalgebra::Point3;
use speedy2d::Graphics2D;
use speedy2d::color::Color;

use crate::shapes::Rectangle;
use crate::transformations::*;


pub fn draw_rectangle(rect: &Rectangle, g: &mut Graphics2D) {
    let points = proj_2d(rect);
    for i in 0..points.len() / 2 {
    let p_2d = (points[i].x, points[i].y);
    let p_2d_next;
    // if end of points, make line to startpoint
    if i == points.len() / 2 - 1 {
        p_2d_next = (points[0].x, points[0].y);
    }
    else {
        p_2d_next = (points[i + 1].x, points[i + 1].y);
    }
    g.draw_line(p_2d, p_2d_next, 2.0, Color::WHITE);
}

    for i in points.len() / 2..points.len() {
        let p_2d = (points[i].x, points[i].y);
        let p_2d_next;
        // if end of points, make line to startpoint
        if i == points.len() - 1 {
            p_2d_next = (points[points.len() / 2].x, points[points.len() / 2].y);
        }
        else {
            p_2d_next = (points[i + 1].x, points[i + 1].y);
        }
        g.draw_line(p_2d, p_2d_next, 2.0, Color::WHITE);
    }

    for i in 0..points.len() / 2 {
        let p_2d = (points[i].x, points[i].y);
        let p_2d_next;
        // if end of points, make line to startpoint
        if i == points.len() - 1 {
            p_2d_next = (points[points.len() / 2].x, points[points.len() / 2].y);
        }
        else {
            p_2d_next = (points[i + points.len() / 2].x, points[i + points.len() / 2].y);
        }
        g.draw_line(p_2d, p_2d_next, 2.0, Color::WHITE);
    }
}
