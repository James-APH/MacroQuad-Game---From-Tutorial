use macroquad::prelude::*;
use std::fs;

pub struct ScoreTracker {
    current_score: u32,
    high_score: u32,
}

pub fn init_score_tracker(score_save_file: &str) -> ScoreTracker {
    ScoreTracker {
        current_score: 0,
        high_score: fs::read_to_string(score_save_file)
            .map_or(Ok(0), |i| i.parse::<u32>())
            .unwrap_or(0),
    }
}

impl ScoreTracker {
    pub fn get_current_score(&self) -> u32 {
        self.current_score
    }

    pub fn get_high_score(&self) -> u32 {
        self.high_score
    }

    pub fn set_current_score(&mut self, new_score: u32) {
        self.current_score = new_score;
    }

    pub fn set_high_score(&mut self, new_high_score: u32) {
        self.high_score = new_high_score
    }

    pub fn draw(&self) {
        draw_text(
            format!("Score: {}", self.get_current_score()).as_str(),
            10.0,
            35.0,
            25.0,
            WHITE,
        );
        let highscore_text = format!("High score: {}", self.get_high_score());
        let text_dimensions = measure_text(highscore_text.as_str(), None, 25, 1.0);
        draw_text(
            highscore_text.as_str(),
            screen_width() - text_dimensions.width - 10.0,
            35.0,
            25.0,
            WHITE,
        );
    }

    pub fn save(&self) {
        if self.current_score > self.high_score {
            fs::write("highscore.dat", self.current_score.to_string()).ok();
        }
    }
}
