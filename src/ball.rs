use std::f32::consts::PI;

use macroquad::*;
use constants::*;

use crate::{Paddle, constants};

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub x_speed: f32,
    pub y_speed: f32,
}

impl Ball {

    pub fn reset(&mut self) {
        let reduce = (PI / 4.0).sin();

        self.x = window::screen_width() / 2.0;
        self.y = window::screen_height() / 2.0;
        self.x_speed = BALL_DEFAULT_SPEED * reduce * ((rand::gen_range::<i32>(0,2) * 2 - 1) as f32);
        self.y_speed = BALL_DEFAULT_SPEED * reduce * ((rand::gen_range::<i32>(0,2) * 2 - 1) as f32);
    }

    pub fn update(&mut self, delta: f32) {

        self.x_speed += BALL_SPEED_INCREASE * delta * self.x_speed.signum();
        self.y_speed += BALL_SPEED_INCREASE * delta * self.y_speed.signum();

        self.x += self.x_speed * delta;
        self.y += self.y_speed * delta;

        if self.y > window::screen_height() - BALL_RADIUS || self.y < BALL_RADIUS { self.y_speed *= -1.0; }
    }

    pub fn draw(&self) {
        shapes::draw_circle(self.x, self.y, BALL_RADIUS, prelude::WHITE);
    }

    pub fn is_touching_paddle(&self, paddle: &Paddle) -> bool {

        let dist_edge = if self.direction() > 0 { window::screen_width() - self.x } else { self.x }; 
        let half_paddle = PADDLE_HEIGHT / 2.0;

        let is_y_touching = (paddle.y + half_paddle - self.y).abs() < BALL_RADIUS + half_paddle;

        if is_y_touching && 
            self.direction() == paddle.side &&
            dist_edge < PADDLE_WALL_DISTANCE + PADDLE_WIDTH + BALL_RADIUS &&
            dist_edge > PADDLE_WALL_DISTANCE  {
            return true;
        }

        false
    }

    pub fn mirror_x_speed(&mut self)
    {
        self.x_speed *= -1.0;
    }

    pub fn distance_from_edge(&self) -> f32 {
        if self.direction() > 0 { window::screen_width() - self.x } else { self.x }
    }

    pub fn is_outside_screen(&self) -> bool {
        self.distance_from_edge() < -BALL_RADIUS
    }

    pub fn direction(&self) -> i32 {
        self.x_speed.signum() as i32
    }
}

impl Default for Ball
{
    fn default() -> Self {
        let mut result = Self { x_speed: 0.0, y_speed: 0.0, x: 0.0, y: 0.0 };
        result.reset();
        result
    }
}