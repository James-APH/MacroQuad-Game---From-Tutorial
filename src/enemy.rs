use crate::game_objects::*;
use macroquad::prelude::*;

struct Square {
    body: Shape,
    color: Color,
}

pub fn init_square(shape_body: Shape, color_type: Color) -> Square {
    Square {
        body: shape_body,
        color: color_type,
    }
}

impl Update for Square {
    fn update(self: &mut Square, dt: f32) {
        let distance = self.body.get_speed() * dt;
        self.body.set_y(distance);
    }
}

impl Draw for Square {
    fn draw(self: &Square) {
        let apothem: f32 = self.body.get_size() / 2.0;
        draw_rectangle(
            self.body.get_x(),
            self.body.get_y(),
            apothem,
            apothem,
            self.color,
        );
    }
}
