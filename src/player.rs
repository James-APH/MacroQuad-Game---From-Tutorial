use crate::game_objects::*;
use macroquad::prelude::*;

pub struct Circle {
    body: Shape,
    color: Color,
}

pub fn init_circle(shape_body: Shape, color_type: Color) -> Circle {
    Circle {
        body: shape_body,
        color: color_type,
    }
}

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
