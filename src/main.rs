mod bullet;
mod enemy;
mod game_objects;
mod player;
mod score;
mod settings;

use crate::bullet::{Bullet, init_bullet};
use crate::enemy::{Enemy, init_enemy};
use crate::game_objects::{Draw, GetBody, Update};
use crate::player::init_player;
use crate::score::init_score_tracker;
use macroquad::prelude::*;

// This macro is used to tell macroquad which function will be run
// when the application starts
#[macroquad::main("My game")]
async fn main() {
    let score_save_file: &str = "highscore.dat";

    let mut score_tracker = init_score_tracker(score_save_file);
    let center = (screen_width() / 2.0, screen_height() / 2.0);

    let mut player = init_player(center, YELLOW);
    let background_color = BLACK;

    let mut enemies = vec![];
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
            player.update(delta_time);

            // Creating new bullets:
            if is_key_pressed(KeyCode::Space) {
                let bullet: Bullet = init_bullet(player.get_body(), RED);
                bullets.push(bullet);
            }

            if rand::gen_range(0, 99) >= 95 {
                let enemy: Enemy = init_enemy(GREEN);
                enemies.push(enemy);
            }

            // Updating squares location
            for enemy in &mut enemies {
                enemy.update(delta_time);
            }
            // Updating bullet location
            for bullet in &mut bullets {
                bullet.update(delta_time);
            }

            // deciding whether to keep a square on the screen depending on where its coords are
            enemies.retain(|enemy| {
                enemy.get_body().get_y() < screen_height() + enemy.get_body().get_size()
            });

            // deciding whether to keep squares / bullets on the screen
            // if they've collided with eachother
            bullets.retain(|bullet| !bullet.get_body().get_collided());
            enemies.retain(|enemy| !enemy.get_body().get_collided());
        }

        // Checking if the circle has collided with any squares

        if enemies
            .iter()
            .any(|enemy| player.get_body().collides_with(enemy.get_body()))
        {
            score_tracker.save();
            game_over = true;
        }

        // Checking if bullets have collided with any squares
        for enemy in enemies.iter_mut() {
            for bullet in bullets.iter_mut() {
                if bullet.get_body().collides_with(enemy.get_body()) {
                    let was_collided = true;
                    bullet.get_body_mut().set_collided(was_collided);
                    enemy.get_body_mut().set_collided(was_collided);
                    score_tracker.set_current_score(
                        score_tracker.get_current_score()
                            + enemy.get_body().get_size().round() as u32,
                    );
                }
            }
        }

        // deleting squares from screen, and repositioning circle
        if game_over && is_key_pressed(KeyCode::Space) {
            game_over = false;
            enemies.clear();
            bullets.clear();
            player.reset(center);
            if score_tracker.get_current_score() > score_tracker.get_high_score() {
                score_tracker.set_high_score(score_tracker.get_current_score());
            }
            score_tracker.set_current_score(0);
        }

        // Drawing everything

        for enemy in &enemies {
            enemy.draw();
        }

        for bullet in &bullets {
            bullet.draw();
        }

        score_tracker.draw();
        player.draw();

        next_frame().await
    }
}
