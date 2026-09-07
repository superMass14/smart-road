pub mod car_struct;
pub mod control_trafic;
pub mod utils;
pub mod stat_struct;
mod collide;
pub use collide::*;
pub use control_trafic::*;
use sdl2::image::LoadTexture;
pub use utils::*;
pub use stat_struct::*;
use car_struct::*;
use sdl2::render::*;
use sdl2::video::*;
use std::path::Path;
use sdl2::rect::*;
use config::*;
use std::time::*;
use std::time::Duration;

impl Car {
    pub fn new(from: Location) -> Self {
        let direction = random_direction();
        let (x, y) = get_coords(from, direction);
        let to = get_destination(from, direction);

        let velocity_x = match from {
            Location::North | Location::South => 0,
            Location::West | Location::East => MIN_SPEED,
        };
        let velocity_y = match from {
            Location::North | Location::South => MIN_SPEED,
            Location::West | Location::East => 0,
        };
        let (width, height) = match from {
            Location::North | Location::South => (CAR_WIDTH as u32, CAR_HEIGHT as u32),
            Location::West | Location::East => (CAR_HEIGHT as u32, CAR_WIDTH as u32),
        };
        let light = match from {
            Location::North =>
                Rect::new(
                    x + LIGHT_WIDTH_MARGIN,
                    y + LIGHT_HEIGHT,
                    LIGHT_WIDTH as u32,
                    LIGHT_HEIGHT as u32
                ),
            Location::South =>
                Rect::new(
                    x + LIGHT_WIDTH_MARGIN,
                    y - LIGHT_HEIGHT,
                    LIGHT_WIDTH as u32,
                    LIGHT_HEIGHT as u32
                ),
            Location::West =>
                Rect::new(
                    x + LIGHT_HEIGHT,
                    y + LIGHT_WIDTH_MARGIN,
                    LIGHT_HEIGHT as u32,
                    LIGHT_WIDTH as u32
                ),
            Location::East =>
                Rect::new(
                    x - LIGHT_HEIGHT,
                    y + LIGHT_WIDTH_MARGIN,
                    LIGHT_HEIGHT as u32,
                    LIGHT_WIDTH as u32
                ),
        };
        let sensor = {
            let (width, height, x, y) = match from {
                Location::North =>
                    (
                        LIGHT_WIDTH,
                        LIGHT_HEIGHT / 3,
                        x,
                        y + ((LIGHT_HEIGHT - LIGHT_HEIGHT / 3) as i32),
                    ),
                Location::South => (LIGHT_WIDTH, LIGHT_HEIGHT / 3, x, y as i32),
                Location::West =>
                    (
                        LIGHT_HEIGHT / 3,
                        LIGHT_WIDTH,
                        x + ((LIGHT_HEIGHT - LIGHT_HEIGHT / 3) as i32),
                        y,
                    ),
                Location::East => (LIGHT_HEIGHT / 3, LIGHT_WIDTH, x as i32, y),
            };
            match from {
                Location::North =>
                    Rect::new(
                        x + LIGHT_WIDTH_MARGIN,
                        y + (height as i32),
                        width as u32,
                        height as u32
                    ),
                Location::South =>
                    Rect::new(
                        x + LIGHT_WIDTH_MARGIN,
                        y - (height as i32),
                        width as u32,
                        height as u32
                    ),
                Location::West =>
                    Rect::new(
                        x + (width as i32),
                        y + LIGHT_WIDTH_MARGIN,
                        width as u32,
                        height as u32
                    ),
                Location::East =>
                    Rect::new(
                        x - (width as i32),
                        y + LIGHT_WIDTH_MARGIN,
                        width as u32,
                        height as u32
                    ),
            }
        };
        let angle = match from {
            Location::East => 0,
            Location::South => 90,
            Location::West => 180,
            Location::North => 270,
        };

        Self {
            image: format!("assets/{:?}_car.png", from),
            from,
            to,
            x,
            y,
            width,
            height,
            arrived: false,
            has_turned: false,
            rotating: false,
            frame_center: Point::new(20, 35),
            rotate_angle: 0.0,
            intersection: false,
            velocity_x,
            velocity_y,
            vel_sign: 1,
            lane: Lane::Line(from),
            light,
            sensor,
            head_lights: Vec::new(),
            head_lights_stop: Vec::new(),
            flip_horizontal: false,
            flip_vertical: false,
            is_blocked: false,
            time: Instant::now(),
            last_time: Duration::new(0, 0),
            distance: get_distance(from, to),
            has_collision: false,
            angle,
            direction,
        }
    }

    pub fn vroom(&mut self, vehicles: Vec<Car>, stats: &mut Stat) {
        match self.from {
            Location::North => {
                if !self.is_blocked {
                    self.y += self.velocity_y;
                    self.x += self.velocity_x;
                    self.light.y += self.velocity_y;
                    self.light.x += self.velocity_x;
                    self.sensor.y += self.velocity_y;
                    self.sensor.x += self.velocity_x;
                }
                if self.y >= NORTH_R_INTERSECT && self.to == Location::East && !self.has_turned {
                    if self.can_turn(vehicles) {
                        self.turn();
                    }
                } else if
                    self.y >= NORTH_L_INTERSECT &&
                    self.to == Location::West &&
                    !self.has_turned
                {
                    if self.can_turn(vehicles) {
                        self.turn();
                    }
                }
            }
            Location::South => {
                if !self.is_blocked {
                    self.y -= self.velocity_y;
                    self.x += self.velocity_x;
                    self.light.y -= self.velocity_y;
                    self.light.x += self.velocity_x;
                    self.sensor.y -= self.velocity_y;
                    self.sensor.x += self.velocity_x;
                }
                if self.y <= SOUTH_R_INTERSECT && self.to == Location::East && !self.has_turned {
                    if self.can_turn(vehicles) {
                        self.turn();
                    }
                } else if
                    self.y <= SOUTH_L_INTERSECT &&
                    self.to == Location::West &&
                    !self.has_turned
                {
                    if self.can_turn(vehicles) {
                        self.turn();
                    }
                }
            }
            Location::West => {
                if !self.is_blocked {
                    self.x += self.velocity_x;
                    self.y += self.velocity_y;
                    self.light.y += self.velocity_y;
                    self.light.x += self.velocity_x;
                    self.sensor.y += self.velocity_y;
                    self.sensor.x += self.velocity_x;
                }
                if self.x >= WEST_R_INTERSECT && self.to == Location::South && !self.has_turned {
                    if self.can_turn(vehicles) {
                        self.turn();
                    }
                } else if
                    self.x >= WEST_L_INTERSECT &&
                    self.to == Location::North &&
                    !self.has_turned
                {
                    if self.can_turn(vehicles) {
                        self.turn();
                    }
                }
            }
            Location::East => {
                if !self.is_blocked {
                    self.x -= self.velocity_x;
                    self.y -= self.velocity_y;
                    self.light.y -= self.velocity_y;
                    self.light.x -= self.velocity_x;
                    self.sensor.y -= self.velocity_y;
                    self.sensor.x -= self.velocity_x;
                }
                if self.x <= EAST_L_INTERSECT && self.to == Location::North && !self.has_turned {
                    if self.can_turn(vehicles) {
                        self.turn();
                    }
                } else if
                    self.x <= EAST_R_INTERSECT &&
                    self.to == Location::South &&
                    !self.has_turned
                {
                    if self.can_turn(vehicles) {
                        self.turn();
                    }
                }
            }
        }
        if
            self.x < -(self.width as i32) ||
            self.y < -(self.width as i32) ||
            self.x + (self.width as i32) > SCREEN_WIDTH + (self.width as i32) ||
            self.y + (self.height as i32) > SCREEN_HEIGHT + (self.width as i32)
        {
            self.last_time = self.time.elapsed();
            self.arrived = true;
            stats.cars.push(self.clone())
        }
    }

    pub fn stop(&mut self) {
        self.set_velocity(0, 0)
    }

    pub fn accelerate(&mut self) {
        let (vel_x, vel_y) = self.get_velocity(MAX_SPEED);
        self.set_velocity(vel_x, vel_y);
    }

    pub fn throttle(&mut self) {
        let (vel_x, vel_y) = self.get_velocity(MIN_SPEED);
        self.set_velocity(vel_x, vel_y);
    }

    pub fn get_velocity(&self, speed: i32) -> (i32, i32) {
        let velocity_x = match self.from {
            Location::North | Location::South => if self.has_turned {
                speed * self.vel_sign
            } else {
                0
            }
            Location::West | Location::East => if self.has_turned {
                0
            } else {
                speed * self.vel_sign
            }
        };
        let velocity_y = match self.from {
            Location::North | Location::South => if self.has_turned {
                0
            } else {
                speed * self.vel_sign
            }
            Location::West | Location::East => if self.has_turned {
                speed * self.vel_sign
            } else {
                0
            }
        };
        (velocity_x, velocity_y)
    }

    pub fn set_velocity(&mut self, vel_x: i32, vel_y: i32) {
        self.velocity_x = vel_x;
        self.velocity_y = vel_y;
    }
    pub fn can_turn(&mut self, mut vehicles: Vec<Car>) -> bool {
        match self.from {
            Location::North => match self.to {
                    Location::West => {
                        self.head_lights.push(
                            Rect::new(
                                self.x - HEAD_LIGHT_HEIGHT,
                                self.y + HEAD_LIGHT_WIDTH_MARGIN,
                                HEAD_LIGHT_HEIGHT as u32,
                                HEAD_LIGHT_WIDTH as u32
                            )
                        );
                    }
                    Location::East => {
                        self.head_lights.push(
                            Rect::new(
                                self.x + HEAD_LIGHT_HEIGHT,
                                self.y + HEAD_LIGHT_WIDTH_MARGIN,
                                HEAD_LIGHT_HEIGHT as u32,
                                HEAD_LIGHT_WIDTH as u32
                            )
                        );
                        self.head_lights_stop.push(
                            Rect::new(
                                self.x - HEAD_LIGHT_HEIGHT,
                                self.y + HEAD_LIGHT_WIDTH_MARGIN,
                                HEAD_LIGHT_HEIGHT as u32,
                                HEAD_LIGHT_WIDTH as u32
                            )
                        );
                    }
                    _ => (),
                }

            Location::South => match self.to {
                Location::West => {
                    self.head_lights.push(
                        Rect::new(
                            self.x - HEAD_LIGHT_HEIGHT,
                            self.y + HEAD_LIGHT_WIDTH_MARGIN,
                            HEAD_LIGHT_HEIGHT as u32,
                            HEAD_LIGHT_WIDTH as u32
                        )
                    );
                    self.head_lights_stop.push(
                        Rect::new(
                            self.x + HEAD_LIGHT_HEIGHT,
                            self.y + HEAD_LIGHT_WIDTH_MARGIN,
                            HEAD_LIGHT_HEIGHT as u32,
                            HEAD_LIGHT_WIDTH as u32
                        )
                    );
                }
                Location::East => {
                    self.head_lights.push(
                        Rect::new(
                            self.x + HEAD_LIGHT_HEIGHT,
                            self.y + HEAD_LIGHT_WIDTH_MARGIN,
                            HEAD_LIGHT_HEIGHT as u32,
                            HEAD_LIGHT_WIDTH as u32
                        )
                    );
                }
                _ => (),
            }
            Location::West => match self.to {
                    Location::North => {
                        self.head_lights.push(
                            Rect::new(
                                self.x + HEAD_LIGHT_WIDTH_MARGIN,
                                self.y - HEAD_LIGHT_HEIGHT,
                                HEAD_LIGHT_WIDTH as u32,
                                HEAD_LIGHT_HEIGHT as u32
                            )
                        );
                        self.head_lights_stop.push(
                            Rect::new(
                                self.x + HEAD_LIGHT_WIDTH_MARGIN,
                                self.y + HEAD_LIGHT_HEIGHT,
                                HEAD_LIGHT_WIDTH as u32,
                                HEAD_LIGHT_HEIGHT as u32
                            )
                        );
                    }
                    Location::South => {
                        self.head_lights.push(
                            Rect::new(
                                self.x + HEAD_LIGHT_WIDTH_MARGIN,
                                self.y + HEAD_LIGHT_HEIGHT,
                                HEAD_LIGHT_WIDTH as u32,
                                HEAD_LIGHT_HEIGHT as u32
                            )
                        );
                    }
                    _ => (),
            }
            Location::East => match self.to {
                    Location::North => {
                        self.head_lights.push(
                            Rect::new(
                                self.x + HEAD_LIGHT_WIDTH_MARGIN,
                                self.y - HEAD_LIGHT_HEIGHT,
                                HEAD_LIGHT_WIDTH as u32,
                                HEAD_LIGHT_HEIGHT as u32
                            )
                        );
                    }
                    Location::South => {
                        self.head_lights.push(
                            Rect::new(
                                self.x + HEAD_LIGHT_WIDTH_MARGIN,
                                self.y + HEAD_LIGHT_HEIGHT,
                                HEAD_LIGHT_WIDTH as u32,
                                HEAD_LIGHT_HEIGHT as u32
                            )
                        );
                        self.head_lights_stop.push(
                            Rect::new(
                                self.x + HEAD_LIGHT_WIDTH_MARGIN,
                                self.y - HEAD_LIGHT_HEIGHT,
                                HEAD_LIGHT_WIDTH as u32,
                                HEAD_LIGHT_HEIGHT as u32
                            )
                        );
                    }
                    _ => (),
            }
        }
        vehicles.retain(|car| car != self);
        for hl in self.head_lights.iter() {
            for new_car in vehicles.iter() {
                if
                    hl.has_intersection(
                        Rect::new(new_car.x, new_car.y, new_car.width, new_car.height)
                    )
                {
                    self.is_blocked = true;
                    self.stop();
                    return false;
                }
            }
        }
        self.is_blocked = false;
        true
    }
    pub fn turn(&mut self) {
        match self.from {
            Location::North if self.to != Location::South => {
                self.image = "assets/north_car_h.png".to_string();
                if self.to == Location::West {
                    self.angle = 0;
                    //-----------lighting-----
                    self.light.x = self.x - LIGHT_HEIGHT;
                    self.light.y -= LIGHT_HEIGHT - LIGHT_WIDTH_MARGIN;
                    self.light.h = LIGHT_WIDTH;
                    self.light.w = LIGHT_HEIGHT;
                    //-----------sensor-----
                    self.sensor.x =
                        self.x - LIGHT_HEIGHT + ((LIGHT_HEIGHT - LIGHT_HEIGHT / 3) as i32);
                    self.sensor.y -= LIGHT_HEIGHT - LIGHT_WIDTH_MARGIN;
                    self.sensor.h = LIGHT_WIDTH;
                    self.sensor.w = LIGHT_HEIGHT / 3;

                    // -------------------
                    self.vel_sign = -1;
                    self.rotate_angle = 90.0;
                    self.has_turned = true;
                    self.flip_horizontal = true;
                } else {
                    self.angle = 180;
                    //-----------lighting-----
                    self.light.x = self.x + LIGHT_HEIGHT;
                    self.light.y -= LIGHT_HEIGHT - LIGHT_WIDTH_MARGIN;
                    self.light.h = LIGHT_WIDTH;
                    self.light.w = LIGHT_HEIGHT;
                    //-----------sensor-----
                    self.sensor.x = self.x + LIGHT_HEIGHT;
                    self.sensor.y -= LIGHT_HEIGHT - LIGHT_WIDTH_MARGIN;
                    self.sensor.h = LIGHT_WIDTH;
                    self.sensor.w = LIGHT_HEIGHT / 3;
                    // -------------------
                    self.vel_sign = 1;
                    self.rotate_angle = -90.0;
                    self.has_turned = true;
                }
                self.width = CAR_HEIGHT as u32;
                self.height = CAR_WIDTH as u32;
            }
            Location::South if self.to != Location::North => {
                self.image = "assets/south_car_h.png".to_string();
                if self.to == Location::West {
                    self.angle = 0;
                    //-----------lighting-----
                    self.light.x = self.x - LIGHT_HEIGHT;
                    self.light.y = self.y + LIGHT_WIDTH_MARGIN;
                    self.light.h = LIGHT_WIDTH;
                    self.light.w = LIGHT_HEIGHT;
                    //-----------sensor-----
                    self.sensor.x = self.x - LIGHT_HEIGHT + (LIGHT_HEIGHT - LIGHT_HEIGHT / 3);
                    self.sensor.y = self.y + LIGHT_WIDTH_MARGIN;
                    self.sensor.h = LIGHT_WIDTH;
                    self.sensor.w = LIGHT_HEIGHT / 3;
                    // -------------------
                    self.vel_sign = -1;
                    self.rotate_angle = -90.0;
                    self.has_turned = true;
                    self.flip_horizontal = true;
                } else {
                    self.angle = 180;
                    //-----------lighting-----
                    self.light.x = self.x + LIGHT_HEIGHT;
                    self.light.y = self.y + LIGHT_WIDTH_MARGIN;
                    self.light.h = LIGHT_WIDTH;
                    self.light.w = LIGHT_HEIGHT;
                    //-----------sensor-----
                    self.sensor.x = self.x + LIGHT_HEIGHT;
                    self.sensor.y = self.y + LIGHT_WIDTH_MARGIN;
                    self.sensor.h = LIGHT_WIDTH;
                    self.sensor.w = LIGHT_HEIGHT / 3;
                    // -------------------
                    self.vel_sign = 1;
                    self.rotate_angle = 90.0;
                    self.has_turned = true;
                }
                self.width = CAR_HEIGHT as u32;
                self.height = CAR_WIDTH as u32;
            }
            Location::West if self.to != Location::East => {
                self.image = "assets/west_car_v.png".to_string();
                if self.to == Location::North {
                    self.angle = 90;
                    //-----------lighting-----
                    self.light.x = self.x + LIGHT_WIDTH_MARGIN;
                    self.light.y = self.y - LIGHT_HEIGHT;
                    self.light.h = LIGHT_HEIGHT;
                    self.light.w = LIGHT_WIDTH;
                    //-----------sensor-----
                    self.sensor.x = self.x + LIGHT_WIDTH_MARGIN;
                    self.sensor.y = self.y - LIGHT_HEIGHT / 3;
                    self.sensor.h = LIGHT_HEIGHT / 3;
                    self.sensor.w = LIGHT_WIDTH;
                    // -------------------
                    self.vel_sign = -1;
                    self.rotate_angle = 0.0;
                    self.has_turned = true;
                } else {
                    self.angle = 270;
                    //-----------lighting-----
                    self.light.x = self.x + LIGHT_WIDTH_MARGIN;
                    self.light.y = self.y + LIGHT_HEIGHT;
                    self.light.h = LIGHT_HEIGHT;
                    self.light.w = LIGHT_WIDTH;
                    //-----------sensor-----
                    self.sensor.x = self.x + LIGHT_WIDTH_MARGIN;
                    self.sensor.y = self.y + LIGHT_HEIGHT;
                    self.sensor.h = LIGHT_HEIGHT / 3;
                    self.sensor.w = LIGHT_WIDTH;
                    self.flip_vertical = true;
                    // -------------------
                    self.vel_sign = 1;
                    self.rotate_angle = 0.0;
                    self.has_turned = true;
                }
                self.width = CAR_WIDTH as u32;
                self.height = CAR_HEIGHT as u32;
            }
            Location::East if self.to != Location::West => {
                self.image = "assets/east_car_v.png".to_string();
                if self.to == Location::North {
                    self.angle = 90;
                    //-----------lighting-----
                    self.light.x = self.x + LIGHT_WIDTH_MARGIN;
                    self.light.y = self.y - LIGHT_HEIGHT;
                    self.light.h = LIGHT_HEIGHT;
                    self.light.w = LIGHT_WIDTH;
                    //-----------sensor-----
                    self.sensor.x = self.x + LIGHT_WIDTH_MARGIN;
                    self.sensor.y = self.y - LIGHT_HEIGHT / 3;
                    self.sensor.h = LIGHT_HEIGHT / 3;
                    self.sensor.w = LIGHT_WIDTH;
                    // -------------------
                    self.vel_sign = 1;
                    self.rotate_angle = 90.0;
                    self.has_turned = true;
                } else {
                    self.angle = 270;
                    //-----------lighting-----
                    self.light.x = self.x + LIGHT_WIDTH_MARGIN;
                    self.light.y = self.y + LIGHT_HEIGHT;
                    self.light.h = LIGHT_HEIGHT;
                    self.light.w = LIGHT_WIDTH;
                    //-----------sensor-----
                    self.sensor.x = self.x + LIGHT_WIDTH_MARGIN;
                    self.sensor.y = self.y + LIGHT_HEIGHT;
                    self.sensor.h = LIGHT_HEIGHT / 3;
                    self.sensor.w = LIGHT_WIDTH;
                    // -------------------
                    self.vel_sign = -1;
                    self.rotate_angle = -90.0;
                    self.has_turned = true;
                    self.flip_vertical = true;
                }
                self.width = CAR_WIDTH as u32;
                self.height = CAR_HEIGHT as u32;
            }
            _ => {}
        }
    }
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let car_texture_innit = canvas.texture_creator();
        let car_path = Path::new(&self.image);
        let car_texture = car_texture_innit.load_texture(car_path).unwrap();
        let car_form = Rect::new(self.x, self.y, self.width, self.height);

        let center = Point::new((self.width / 2) as i32, (self.height / 2) as i32);
        canvas
            .copy_ex(
                &car_texture,
                None,
                Some(car_form),
                0.0,
                center,
                self.flip_horizontal,
                self.flip_vertical
            )
            .unwrap();
        // canvas.present();
        // canvas.set_draw_color(Color::RGB(255, 140, 0));
        // let _ = canvas.fill_rect(self.light);
        // canvas.set_draw_color(Color::RGB(255, 0, 0));
        // let _ = canvas.fill_rect(self.sensor);
        // for hl in self.head_lights.iter() {
        //     canvas.set_draw_color(Color::RGB(0, 255, 255));
        //     let _ = canvas.fill_rect(*hl);
        // }
        // for hl in self.head_lights_stop.iter() {
        //     canvas.set_draw_color(Color::RGB(0, 255, 255));
        //     let _ = canvas.fill_rect(*hl);
        // }
    }
}
