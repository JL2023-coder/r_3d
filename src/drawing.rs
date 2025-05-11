use nalgebra::{Const, OPoint};
use speedy2d::dimen::Vector2;
use speedy2d::{color, Graphics2D};
use speedy2d::color::Color;

// Define the connections for a standard rectangle
const CUBE_EDGES_4: [(usize, usize); 12] = [
    (0, 1), (1, 2), (2, 3), (3, 0), // First face
    (4, 5), (5, 6), (6, 7), (7, 4), // Second face
    (0, 4), (1, 5), (2, 6), (3, 7), // Connecting edges
];

// Faces for rectangle
const CUBE_EDGES_6: [(usize, usize, usize, usize); 6] = [
    (0, 1, 2, 3), // Down face
    (4, 5, 6, 7), // Up face
    (0, 1, 5, 4), // Front face
    (2, 3, 7, 6), // Back face
    (1, 2, 6, 5), // Right face
    (0, 3, 7, 4), // Left edges
];
// Draws lines to rectangle, does not take in a rectangle but instead 8 points
// For now the calculation of where points should be in screen happenes outside of draw method
pub fn draw_rectangle_outline(points: [OPoint<f32, Const<3>>; 8], g: &mut Graphics2D) {
    for (start_index, end_index) in CUBE_EDGES_4.iter() {
        let p1 = &points[*start_index];
        let p2 = &points[*end_index];

        let p1_2d = (p1.x, p1.y);
        let p2_2d = (p2.x, p2.y);

        g.draw_line(p1_2d, p2_2d, 2.0, Color::WHITE);
    }
}

// Fills in rectangle
// Needs to be fixed
/* pub fn draw_rectangle_full(points: [OPoint<f32, Const<3>>; 8], g: &mut Graphics2D) {
    for (p1, p2, p3, p4) in CUBE_EDGES_6 {
        let right_corner_top: Vector2<f32> = Vector2::new(*&points[p1].x, *&points[p1].y);
        let right_corner_bot: Vector2<f32> = Vector2::new(*&points[p2].x, *&points[p2].y);
        let left_corner_top: Vector2<f32> = Vector2::new(*&points[p3].x, *&points[p3].y);
        let left_corner_bot: Vector2<f32> = Vector2::new(*&points[p4].x, *&points[p4].y); 
     

        let polygon = Polygon::new(&vec![left_corner_bot, right_corner_bot, right_corner_top, left_corner_top]);
        g.draw_polygon(&polygon, Vector2::new(0.0, 0.0), Color::RED);
    }
} */

// Draws one face of the rectangle
pub fn draw_rectangle_face(points: [OPoint<f32, Const<3>>; 8], g: &mut Graphics2D) {
    let (p1, p2, p3, p4) = CUBE_EDGES_6[5];

    let vertices = [
        Vector2::new(points[p1].x, points[p1].y),
        Vector2::new(points[p2].x, points[p2].y),
        Vector2::new(points[p3].x, points[p3].y),
        Vector2::new(points[p4].x, points[p4].y)
    ];

   g.draw_quad(vertices, Color::RED); 
}




