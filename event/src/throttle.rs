use std::time::*;

pub struct Throttle {
    pub arrow_up: (Instant, bool),
    pub arrow_dow: (Instant, bool),
    pub arrow_left: (Instant, bool),
    pub arrow_right: (Instant, bool),
    pub key_r: (Instant, bool),
}

impl Throttle {
    pub fn new() -> Self {
        Self {
            arrow_up: (Instant::now(), false),
            arrow_dow: (Instant::now(), false),
            arrow_left: (Instant::now(), false),
            arrow_right: (Instant::now(), false),
            key_r: (Instant::now(), false),
        }
    }
}
