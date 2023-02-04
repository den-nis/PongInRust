mod ball;
mod constants;
mod paddle;
mod state;

use ball::Ball;
use paddle::Paddle;
use state::State;

use constants::*;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong!".to_owned(),
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    rand::srand(macroquad::miniquad::date::now() as _);

    let center = screen_height() / 2.0;

    let mut paddles: [Paddle; 2] = [
        Paddle { y: center - PADDLE_HEIGHT / 2.0, side: -1, up_key: KeyCode::W,  down_key: KeyCode::S, speed: 0.0 },
        Paddle { y: center - PADDLE_HEIGHT / 2.0, side:  1, up_key: KeyCode::Up, down_key: KeyCode::Down, speed: 0.0 },
    ];

    let mut ball = Ball::default();
    let mut state = State::default();

    loop {
        
        let delta = get_frame_time();

        update(delta, &mut ball, &mut state, &mut paddles);

        clear_background(BLACK);

        ball.draw();

        for paddle in &paddles {
            paddle.draw();
        }

        draw_text(&format!("Score: {}-{}", state.score_left, state.score_right), 3.0, 45.0, 30.0, WHITE);
        draw_text(&format!("FPS {}", get_fps()), 3.0, 20.0, 30.0, WHITE);
        draw_countdown(ball.x, ball.y - 100.0, &state);

        next_frame().await
    }
}

fn draw_countdown(x: f32, y: f32, state: &State) {
    if state.warmup > 0.0 {
        draw_text(&format!("Starting in {}", state.warmup as i32 + 1), x - 85.0, y, 30.0, WHITE);
    }
}

fn update(delta: f32, ball: &mut Ball, state: &mut State, paddles: &mut [Paddle; 2]) {

    update_paddles(delta, paddles);

    if state.warmup > 0.0 {
        state.warmup -= delta;
    } else {

        ball.update(delta);
        
        for paddle in paddles {
            if ball.is_touching_paddle(paddle) {
                ball.mirror_x_speed();
            }
        }

        if ball.is_outside_screen() {
            if ball.direction() > 0 { state.score_left += 1 } else { state.score_right += 1 }
            state.warmup = WARMUP_TIME;
            ball.reset();
        }
    }
}

fn update_paddles(delta: f32, paddles: &mut [Paddle; 2]) {
    for paddle in paddles {
        paddle.update(delta);
    }
}