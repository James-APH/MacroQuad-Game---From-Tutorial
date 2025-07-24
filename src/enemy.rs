use crate::game_objects::*;
use macroquad::prelude::*;

pub struct Enemy {
    body: Body,
    color: Color,
}

pub fn init_enemy(color_type: Color) -> Enemy {
    let size = rand::gen_range(16.0, 64.0);
    let speed = rand::gen_range(50.0, 150.0);
    let x = rand::gen_range(size / 2.0, screen_width() - size / 2.0);
    let y = -size;
    Enemy {
        body: init_body(size, speed, (x, y)),
        color: color_type,
    }
}

impl Update for Enemy {
    fn update(self: &mut Enemy, dt: f32) {
        let distance = self.body.get_speed() * dt;
        self.body.set_y(self.body.get_y() + distance);
    }
}

impl Draw for Enemy {
    fn draw(self: &Enemy) {
        let width: f32 = self.body.get_size();
        draw_rectangle(
            self.body.get_x(),
            self.body.get_y(),
            width,
            width,
            self.color,
        );
    }
}

impl GetBody for Enemy {
    fn get_body(self: &Enemy) -> &Body {
        &self.body
    }

    fn get_body_mut(self: &mut Enemy) -> &mut Body {
        &mut self.body
    }
}
