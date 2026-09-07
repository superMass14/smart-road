mod throttle;
use std::time::*;
use config::*;
use mouse::MouseButton;
use sdl2::render::Canvas;
use sdl2::video::Window;
pub use throttle::*;
use car::{ can_move, car_struct::*, format_location, random_spawn, CollideZones };
use sdl2::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn handle_event(
    event_pump: &mut EventPump,
    vehicles: &mut Vec<Car>,
    collide_zones: &mut CollideZones,
    throttle: &mut Throttle,
    video_subsystem: &VideoSubsystem,
    stats_canvas: &mut Option<Canvas<Window>>,
    display_stats: &mut bool
) -> Result<bool, String> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                if !*display_stats {
                    let window2 = video_subsystem
                        .window("Stats", (SCREEN_WIDTH / 2) as u32, (SCREEN_HEIGHT / 2) as u32)
                        .position_centered()
                        .build()
                        .unwrap();

                    let canvas2 = window2.into_canvas().build().unwrap();
                    *stats_canvas = Some(canvas2);
                    *display_stats = true;
                }
            }

            Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                if let Some(ref mut _canvas2) = stats_canvas {
                    if mouse_btn == MouseButton::Left {
                        if x >= 140 && x <= 350 && y >= 220 && y <= 300 {
                            return Err("end".to_string());
                        }
                    }
                }
            }

            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                if
                    throttle.arrow_dow.0.elapsed() >= Duration::from_millis(WAIT_TIME) ||
                    !throttle.arrow_dow.1
                {
                    let car = Car::new(Location::North);
                    let zone = format_location(car.from, car.to);
                    if can_move(&car, &vehicles) {
                        vehicles.push(car.clone());
                        collide_zones.add_car(zone, &car);
                        throttle.arrow_dow.0 = Instant::now();
                    }
                    throttle.arrow_dow.1 = true;
                }
            }

            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                if
                    throttle.arrow_up.0.elapsed() >= Duration::from_millis(WAIT_TIME) ||
                    !throttle.arrow_up.1
                {
                    let car = Car::new(Location::South);
                    let zone = format_location(car.from, car.to);
                    if can_move(&car, &vehicles) {
                        vehicles.push(car.clone());
                        collide_zones.add_car(zone, &car);
                        throttle.arrow_up.0 = Instant::now();
                    }
                    throttle.arrow_up.1 = true;
                }
            }

            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                if
                    throttle.arrow_left.0.elapsed() >= Duration::from_millis(WAIT_TIME) ||
                    !throttle.arrow_left.1
                {
                    let car = Car::new(Location::East);
                    let zone = format_location(car.from, car.to);
                    if can_move(&car, &vehicles) {
                        vehicles.push(car.clone());
                        collide_zones.add_car(zone, &car);
                        throttle.arrow_left.0 = Instant::now();
                    }
                    throttle.arrow_left.1 = true;
                }
            }

            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                if
                    throttle.arrow_right.0.elapsed() >= Duration::from_millis(WAIT_TIME) ||
                    !throttle.arrow_right.1
                {
                    let car = Car::new(Location::West);
                    let zone = format_location(car.from, car.to);
                    if can_move(&car, &vehicles) {
                        vehicles.push(car.clone());
                        collide_zones.add_car(zone, &car);
                        throttle.arrow_right.0 = Instant::now();
                    }
                    throttle.arrow_right.1 = true;
                }
            }

            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                if
                    throttle.key_r.0.elapsed() >= Duration::from_millis(WAIT_TIME) ||
                    !throttle.key_r.1
                {
                    let car = Car::new(random_spawn());
                    let zone = format_location(car.from, car.to);
                    if can_move(&car, &vehicles) {
                        vehicles.push(car.clone());
                        collide_zones.add_car(zone, &car);
                        throttle.key_r.0 = Instant::now();
                    }
                    throttle.key_r.1 = true;
                }
            }

            _ => {}
        }
    }
    return Ok(true);
}
