use crate::game_objects::*;
use macroquad::prelude::*;

pub struct Bullet {
    body: Shape,
    color: Color,
}

pub fn init_bullet(shape_body: Shape, color_type: Color) -> Bullet {
    Bullet {
        body: shape_body,
        color: color_type,
    }
}

impl Update for Bullet {
    fn update(self: &mut Bullet, dt: f32) {
        let distance = self.body.get_speed() * dt;
    }
}

impl Draw for Bullet {
    fn draw(self: &Bullet) {
        let radius: f32 = self.body.get_size() / 2.0;
        draw_circle(self.body.get_x(), self.body.get_y(), radius, self.color);
    }
}
