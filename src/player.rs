use crate::game_objects::*;
use macroquad::prelude::*;

pub struct Circle {
    body: Shape,
    color: Color,
}

/// Constructor for circle
pub fn init_circle(shape_body: Shape, color_type: Color) -> Circle {
    Circle {
        body: shape_body,
        color: color_type,
    }
}

/// Fuction to update circles position
impl Update for Circle {
    fn update(self: &mut Circle, dt: f32) {
        let distance = self.body.get_speed() * dt;
        if is_key_down(KeyCode::Right) {
            self.body.set_x(distance);
        }
        if is_key_down(KeyCode::Left) {
            self.body.set_x(-distance);
        }
        if is_key_down(KeyCode::Down) {
            self.body.set_y(distance);
        }
        if is_key_down(KeyCode::Up) {
            self.body.set_y(-distance)
        }

        // makes sure x and y are never above or below the window
        self.body
            .set_x(clamp(self.body.get_x(), 0.0, screen_width()));
        self.body
            .set_y(clamp(self.body.get_y(), 0.0, screen_height()));
    }
}

/// Function to draw circle to the screen
impl Draw for Circle {
    fn draw(self: &Circle) {
        let radius: f32 = self.body.get_size() / 2.0;
        draw_circle(self.body.get_x(), self.body.get_y(), radius, self.color);
    }
}
