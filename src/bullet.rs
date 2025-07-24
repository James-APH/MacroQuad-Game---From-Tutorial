use crate::game_objects::*;
use macroquad::prelude::*;

pub struct Bullet {
    body: Body,
    color: Color,
}

pub fn init_bullet(parent_body: &Body, color_type: Color) -> Bullet {
    Bullet {
        body: init_body(
            parent_body.get_size() / 2.0,
            parent_body.get_speed(),
            (parent_body.get_x(), parent_body.get_y()),
        ),
        color: color_type,
    }
}

impl Update for Bullet {
    fn update(self: &mut Bullet, dt: f32) {
        let distance = self.body.get_speed() * dt;
        self.body.set_y(self.body.get_y() - distance);
    }
}

impl Draw for Bullet {
    fn draw(self: &Bullet) {
        let radius: f32 = self.body.get_size() / 2.0;
        draw_circle(self.body.get_x(), self.body.get_y(), radius, self.color);
    }
}

impl GetBody for Bullet {
    fn get_body(self: &Bullet) -> &Body {
        &self.body
    }

    fn get_body_mut(self: &mut Bullet) -> &mut Body {
        &mut self.body
    }
}
