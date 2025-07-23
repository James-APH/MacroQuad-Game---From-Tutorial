use macroquad::prelude::*;
mod game_objects;
mod player;
mod settings;
use std::fs;

struct Square {
    inner: Shape,
}

impl Update for Square {
    fn update(self: &mut Square, dt: f32) {}
}

// This macro is used to tell macroquad which function will be run
// when the application starts
#[macroquad::main("My game")]
async fn main() {
    rand::srand(miniquad::date::now as u64);
    let mut score: u32 = 0;
    let mut high_score: u32 = fs::read_to_string("highscore.dat")
        .map_or(Ok(0), |i| i.parse::<u32>())
        .unwrap_or(0);
    let center = (screen_width() / 2.0, screen_height() / 2.0);

    let mut circle_body = init_shape(32., 200., center);
    let mut circle = init_circle(circle_body, YELLOW);

    let mut squares = vec![];
    let mut bullets: Vec<Shape> = vec![];

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
                bullets.push(Shape {
                    x: circle.x,
                    y: circle.y,
                    speed: circle.speed * 2.0,
                    size: 5.0,
                    collided: false,
                });
            }

            // using rand::gen_range() to determine whether to add a new
            // square.
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    collided: false,
                });
            }

            // Updating squares location
            for square in &mut squares {
                square.y += square.speed * delta_time;
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
        if squares.iter().any(|square| circle.collides_with(square)) {
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
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            score = 0;
            game_over = false;
        }

        // Drawing everything
        draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }

        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size, RED);
        }

        draw_text(format!("Score: {score}").as_str(), 10.0, 35.0, 25.0, WHITE);
        let highscore_text = format!("High score: {high_score}");
        let text_dimensions = measure_text(highscore_text.as_str(), None, 25, 1.0);
        draw_text(
            highscore_text.as_str(),
            screen_width() - text_dimensions.width - 10.0,
            35.0,
            25.0,
            WHITE,
        );

        // printing game over message
        if game_over {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }

        next_frame().await
    }
}
