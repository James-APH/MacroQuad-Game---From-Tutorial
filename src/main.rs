mod bullet;
mod enemy;
mod game_objects;
mod player;
mod score;
mod settings;

use crate::bullet::init_bullet;
use crate::game_objects::{init_shape, Body, Draw, Shape};
use crate::player::init_circle;
use macroquad::prelude::*;
use std::fs;

use crate::{enemy::init_square, game_objects::Update};

// This macro is used to tell macroquad which function will be run
// when the application starts
#[macroquad::main("My game")]
async fn main() {
    rand::srand(miniquad::date::now as u64);

    let center = (screen_width() / 2.0, screen_height() / 2.0);

    let mut circle_body = init_shape(32., 200., center);
    let mut circle = init_circle(circle_body, YELLOW);

    let mut squares = vec![];
    let mut bullets = vec![];

    // Determines how quickly the circle should move
    let mut game_over = false;

    loop {
        // even if clear_background() is not used explicitly, the screen
        // will be cleared with black at the start of each frame.
        clear_background(DARKBLUE);
        if !game_over {
            // gets the t8ime in seconds that has passed since the last frame
            let delta_time = get_frame_time();
            circle.update(delta_time);

            // Creating new bullets:
            if is_key_pressed(KeyCode::Space) {
                bullets.push(init_bullet(circle.body(), RED));
            }

            // using rand::gen_range() to determine whether to add a new
            // square.
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(init_square(GREEN));
            }

            // Updating squares location
            for square in &mut squares {
                square.update(delta_time);
            }
            // Updating bullet location
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time
            }

            // deciding whether to keep a square on the screen depending on where its coords are
            squares.retain(|square| square.y < screen_height() + square.size);

            // deciding whether to keep squares / bullets on the screen
            // if they've collided with eachother
            bullets.retain(|bullet| !bullet.collided);
            squares.retain(|square| !square.collided);
        }

        // Checking if the circle has collided with any squares
        if squares
            .iter()
            .any(|square| circle.body.collides_with(square.body))
        {
            if score == high_score {
                fs::write("highscore.dat", high_score.to_string()).ok();
            }
            game_over = true;
        }

        // Checking if bullets have collided with any squares
        for square in squares.iter_mut() {
            for bullet in bullets.iter_mut() {
                if bullet.collides_with(square) {
                    bullet.collided = true;
                    square.collided = true;
                    score += square.size.round() as u32;
                    high_score = high_score.max(score);
                }
            }
        }

        // deleting squares from screen, and repositioning circle
        if game_over && is_key_pressed(KeyCode::Space) {
            squares.clear();
            bullets.clear();
            circle = screen_width() / 2.0;
            circle = screen_height() / 2.0;
            score = 0;
            game_over = false;
        }

        // Drawing everything

        for square in &squares {
            square.draw();
        }

        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size, RED);
        }

        // printing game over message
        if game_over {}

        next_frame().await
    }
}
