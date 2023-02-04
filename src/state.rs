use crate::constants::*;

pub struct State {
    pub warmup: f32,
    pub score_left: i32,
    pub score_right: i32,
}

impl Default for State
{
    fn default() -> Self {
        Self { warmup: WARMUP_TIME, score_left: 0, score_right: 0, }
    }
}