use macroquad::prelude::*;

use crate::constants::*;

pub struct Paddle {
    pub side: i32,
    pub y: f32,
    pub speed: f32,
    pub up_key: KeyCode,
    pub down_key: KeyCode 
}

impl Paddle {
    
    pub fn update(&mut self, delta: f32) {

        //Move paddles using controls
        let direction = is_key_down(self.down_key) as i8 - is_key_down(self.up_key) as i8;
        self.y += direction as f32 * PADDLE_SPEED * delta;

        //Prevent paddle from exiting the screen
        self.speed = 0.0;
        if self.y + PADDLE_HEIGHT > screen_height() { self.y = screen_height()- PADDLE_HEIGHT; return; }
        if self.y < 0.0 { self.y = 0.0; return; }
        self.speed = direction as f32 * PADDLE_SPEED;
    }

    pub fn draw(&self) {
        let distance = PADDLE_WALL_DISTANCE + PADDLE_WIDTH / 2.0;
        let x = if self.side > 0 { screen_width() - distance} else { distance };

        draw_rectangle(x - PADDLE_WIDTH / 2.0, self.y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
    }
}