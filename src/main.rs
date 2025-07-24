mod bullet;
mod enemy;
mod game_objects;
mod player;
mod score;
mod settings;

use crate::bullet::{Bullet, init_bullet};
use crate::enemy::{Square, init_square};
use crate::game_objects::{Body, Draw, Update};
use crate::player::init_circle;
use crate::score::init_score_tracker;
use macroquad::prelude::*;

// This macro is used to tell macroquad which function will be run
// when the application starts
#[macroquad::main("My game")]
async fn main() {
    let score_save_file: &str = "highscore.dat";

    let mut score_tracker = init_score_tracker(score_save_file);
    let center = (screen_width() / 2.0, screen_height() / 2.0);

    let mut circle = init_circle(center, YELLOW);
    let background_color = BLACK;

    let mut squares = vec![];
    let mut bullets = vec![];

    let mut game_over = false;

    rand::srand(miniquad::date::now as u64);
    loop {
        clear_background(background_color);

        //
        //
        // DEBUG
        //
        //

        // even if clear_background() is not used explicitly, the screen
        // will be cleared with black at the start of each frame.

        if !game_over {
            let delta_time = get_frame_time();
            circle.update(delta_time);

            // Creating new bullets:
            if is_key_pressed(KeyCode::Space) {
                let bullet: Bullet = init_bullet(circle.get_body(), RED);
                bullets.push(bullet);
            }

            if rand::gen_range(0, 99) >= 95 {
                let square: Square = init_square(GREEN);
                squares.push(square);
            }

            // Updating squares location
            for square in &mut squares {
                square.update(delta_time);
            }
            // Updating bullet location
            for bullet in &mut bullets {
                bullet.update(delta_time);
            }

            // deciding whether to keep a square on the screen depending on where its coords are
            squares.retain(|square| {
                square.get_body().get_y() < screen_height() + square.get_body().get_size()
            });

            // deciding whether to keep squares / bullets on the screen
            // if they've collided with eachother
            bullets.retain(|bullet| !bullet.get_body().get_collided());
            squares.retain(|square| !square.get_body().get_collided());
        }

        // Checking if the circle has collided with any squares

        if squares
            .iter()
            .any(|square| circle.get_body().collides_with(square.get_body()))
        {
            score_tracker.save();
            game_over = true;
        }

        // Checking if bullets have collided with any squares
        for square in squares.iter_mut() {
            for bullet in bullets.iter_mut() {
                if bullet.get_body().collides_with(square.get_body()) {
                    let was_collided = true;
                    bullet.get_body_mut().set_collided(was_collided);
                    square.get_body_mut().set_collided(was_collided);
                    score_tracker.set_current_score(
                        score_tracker.get_current_score()
                            + square.get_body().get_size().round() as u32,
                    );
                }
            }
        }

        //
        //
        // DEBUG:
        //
        //

        // deleting squares from screen, and repositioning circle
        if game_over && is_key_pressed(KeyCode::Space) {
            squares.clear();
            bullets.clear();
            circle.reset(center);
            score_tracker.set_current_score(0);
            game_over = false;
        }

        // Drawing everything

        for square in &squares {
            square.draw();
        }

        for bullet in &bullets {
            bullet.draw();
        }

        score_tracker.draw();
        circle.draw();

        next_frame().await
    }
}
