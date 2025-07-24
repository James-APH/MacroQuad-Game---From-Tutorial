use crate::game_objects::*;
use macroquad::prelude::*;

pub struct Circle {
    body: Shape,
    color: Color,
}

/// Constructor for circle
pub fn init_circle(coords: (f32, f32), color_type: Color) -> Circle {
    Circle {
        body: init_shape(32., 200., coords),
        color: color_type,
    }
}

/// Fuction to update circles position
impl Update for Circle {
    fn update(self: &mut Circle, dt: f32) {
        let distance = self.body.get_speed() * dt;
        if is_key_down(KeyCode::Right) {
            self.body.set_x(self.body.get_x() + distance);
        }
        if is_key_down(KeyCode::Left) {
            self.body.set_x(self.body.get_x() - distance);
        }
        if is_key_down(KeyCode::Down) {
            self.body.set_y(self.body.get_y() + distance);
        }
        if is_key_down(KeyCode::Up) {
            self.body.set_y(self.body.get_y() - distance);
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
        println!(
            "Drawing player at ({}, {})",
            self.body.get_x(),
            self.body.get_y()
        );
        draw_circle(self.body.get_x(), self.body.get_y(), radius, self.color);
    }
}

impl Body for Circle {
    fn get_body(self: &Circle) -> &Shape {
        &self.body
    }

    fn get_body_mut(self: &mut Circle) -> &mut Shape {
        &mut self.body
    }
}

impl Circle {
    pub fn reset(self: &mut Circle, coords: (f32, f32)) {
        self.body.set_x(coords.0);
        self.body.set_y(coords.1);
    }
}
