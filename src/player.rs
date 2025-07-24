use crate::game_objects::*;
use macroquad::prelude::*;

pub struct Player {
    body: Body,
    color: Color,
}

/// Constructor for circle
pub fn init_player(coords: (f32, f32), color_type: Color) -> Player {
    Player {
        body: init_body(32., 200., coords),
        color: color_type,
    }
}

/// Fuction to update circles position
impl Update for Player {
    fn update(self: &mut Player, dt: f32) {
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
impl Draw for Player {
    fn draw(self: &Player) {
        let radius: f32 = self.body.get_size() / 2.0;
        draw_circle(self.body.get_x(), self.body.get_y(), radius, self.color);
    }
}

impl GetBody for Player {
    fn get_body(self: &Player) -> &Body {
        &self.body
    }

    fn get_body_mut(self: &mut Player) -> &mut Body {
        &mut self.body
    }
}

impl Player {
    pub fn reset(self: &mut Player, coords: (f32, f32)) {
        self.body.set_x(coords.0);
        self.body.set_y(coords.1);
    }
}
