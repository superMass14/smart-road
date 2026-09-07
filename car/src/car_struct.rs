use sdl2::rect::{ Point, Rect };
use std::time::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Car {
    pub image: String,
    pub from: Location,
    pub to: Location,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub arrived: bool,
    pub has_turned: bool,
    pub rotating: bool,
    pub frame_center: Point,
    pub rotate_angle: f64,
    pub intersection: bool,
    pub velocity_x: i32,
    pub velocity_y: i32,
    pub vel_sign: i32,
    pub lane: Lane,
    pub light: Rect,
    pub sensor: Rect,
    pub head_lights: Vec<Rect>,
    pub head_lights_stop: Vec<Rect>,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
    pub is_blocked: bool,
    pub time: Instant,
    pub last_time: Duration,
    pub distance: f64,
    pub has_collision: bool,
    pub angle: u32,
    pub direction: Direction
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Location {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane {
    Line(Location),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Right,
    Straight,
    Left,
}
