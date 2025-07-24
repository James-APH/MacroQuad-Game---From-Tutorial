use crate::game_objects::*;
use macroquad::prelude::*;

pub struct Square {
    body: Shape,
    color: Color,
}

pub fn init_square(color_type: Color) -> Square {
    let size = rand::gen_range(16.0, 64.0);
    let speed = rand::gen_range(50.0, 150.0);
    let x = rand::gen_range(size / 2.0, screen_width() - size / 2.0);
    let y = -size;
    Square {
        body: init_shape(size, speed, (x, y)),
        color: color_type,
    }
}

impl Update for Square {
    fn update(self: &mut Square, dt: f32) {
        let distance = self.body.get_speed() * dt;
        self.body.set_y(self.body.get_y() + distance);
    }
}

impl Draw for Square {
    fn draw(self: &Square) {
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

impl Body for Square {
    fn get_body(self: &Square) -> &Shape {
        &self.body
    }

    fn get_body_mut(self: &mut Square) -> &mut Shape {
        &mut self.body
    }
}
