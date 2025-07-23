use macroquad::prelude::*;

pub trait Access_Modifiers {
    fn get_x_coord() -> f32;
    fn get_y_coord() -> f32;
    fn set_x_coord(distance: f32) -> f32;
    fn set_y_coord(distance: f32) -> f32;
}

pub trait Update {
    fn update(&mut self, dt: f32);
}

pub struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
}

pub fn init_shape(body_size: f32, body_speed: f32, body_coords: (f32, f32)) -> Shape {
    Shape {
        size: body_size,
        speed: body_speed,
        x: body_coords.0,
        y: body_coords.1,
        collided: false,
    }
}

impl Shape {
    pub fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    pub fn rect(&self) -> Rect {
        // origin of Rect is also from the top left corner, so
        // we subtract half its size from both x and y
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn get_collided(&self) -> f32 {
        self.collided
    }

    pub fn set_collided(&self, was_collision: bool) {
        self.collided = was_collision;
    }

    pub fn set_x(&self, distance: f32) {
        self.x += distance
    }

    pub fn set_y(&self, distance: f32) {
        self.y += distance
    }

    pub fn get_x(&self) {
        self.x
    }

    pub fn get_y(&self) {
        self.y
    }
}
