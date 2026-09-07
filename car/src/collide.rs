use sdl2::pixels::Color;

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CollideZone {
    pub names: Vec<String>,
    pub collide: Rect,
    pub cars: Vec<Car>,
}

pub struct CollideZones(pub Vec<CollideZone>);

impl CollideZone {
    pub fn new(names: Vec<String>) -> Self {
        if names.len() == 2 {
            match (names[0].as_str(), names[1].as_str()) {
                ("e_w", "s_n") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 + 70, SCREEN_HEIGHT / 2 - 75, 20, 20),
                        cars: Vec::new(),
                    },

                ("e_w", "w_n") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 + 23, SCREEN_HEIGHT / 2 - 75, 20, 20),
                        cars: Vec::new(),
                    },

                ("e_w", "n_e") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 - 33, SCREEN_HEIGHT / 2 - 75, 20, 20),
                        cars: Vec::new(),
                    },
                ("e_w", "n_s") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 - 88, SCREEN_HEIGHT / 2 - 75, 20, 20),
                        cars: Vec::new(),
                    },
                //-----------------
                ("e_s", "s_n") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 + 70, SCREEN_HEIGHT / 2 - 28, 20, 20),
                        cars: Vec::new(),
                    },

                ("s_w", "n_s") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 - 88, SCREEN_HEIGHT / 2 - 28, 20, 20),
                        cars: Vec::new(),
                    },
                //--------
                ("n_e", "s_n") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 + 70, SCREEN_HEIGHT / 2 + 10, 20, 20),
                        cars: Vec::new(),
                    },

                ("w_n", "n_s") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 - 88, SCREEN_HEIGHT / 2 + 10, 20, 20),
                        cars: Vec::new(),
                    },
                //--------------
                ("w_e", "s_n") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 + 70, SCREEN_HEIGHT / 2 + 48, 20, 20),
                        cars: Vec::new(),
                    },

                ("w_e", "s_w") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 + 23, SCREEN_HEIGHT / 2 + 48, 20, 20),
                        cars: Vec::new(),
                    },

                ("w_e", "e_s") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 - 33, SCREEN_HEIGHT / 2 + 48, 20, 20),
                        cars: Vec::new(),
                    },
                ("w_e", "n_s") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 - 88, SCREEN_HEIGHT / 2 + 48, 20, 20),
                        cars: Vec::new(),
                    },
                _ => { panic!("problem in collide len == 2 ") }
            }
        } else {
            match (names[0].as_str(), names[1].as_str(), names[2].as_str()) {
                ("n_e", "s_w", "e_s") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 - 33, SCREEN_HEIGHT / 2 - 28, 20, 20),
                        cars: Vec::new(),
                    },

                ("n_e", "s_w", "w_n") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 + 23, SCREEN_HEIGHT / 2 + 10, 20, 20),
                        cars: Vec::new(),
                    },

                ("n_e", "w_n", "e_s") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 - 33, SCREEN_HEIGHT / 2 + 10, 20, 20),
                        cars: Vec::new(),
                    },
                ("e_s", "w_n", "s_w") =>
                    Self {
                        names,
                        collide: Rect::new(SCREEN_WIDTH / 2 + 23, SCREEN_HEIGHT / 2 - 28, 20, 20),
                        cars: Vec::new(),
                    },
                _ => { panic!("problem in collide len == 3 ") }
            }
        }
    }
    pub fn remove(&mut self) {
        if self.cars.len() > 0 {
            self.cars.remove(0);
        }
    }
    pub fn add_car(&mut self, new_car: Car) {
        self.cars.push(new_car)
    }
}

impl CollideZones {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, zone: CollideZone) {
        self.0.push(zone)
    }

    pub fn add_car(&mut self, zone: String, new_car: &Car) {
        for zones in self.0.iter_mut() {
            if zones.names.contains(&zone) {
                zones.add_car(new_car.clone());
            }
        }
    }

    pub fn draw_collide_zone(&mut self, canvas: &mut Canvas<Window>) {
        for zone in &self.0 {
            canvas.set_draw_color(Color::WHITE);
            let _ = canvas.fill_rect(zone.collide);
        }
        // canvas.present();
    }

    pub fn init(&mut self) {
        let intersection_zones: Vec<Vec<&str>> = vec![
            vec!["e_w", "s_n"],
            vec!["e_w", "w_n"],
            vec!["e_w", "n_e"],
            vec!["e_w", "n_s"],

            vec!["e_s", "s_n"],
            vec!["e_s", "w_n", "s_w"],
            vec!["n_e", "s_w", "e_s"],
            vec!["s_w", "n_s"],

            vec!["n_e", "s_n"],
            vec!["n_e", "s_w", "w_n"],
            vec!["n_e", "w_n", "e_s"],
            vec!["w_n", "n_s"],

            vec!["w_e", "s_n"],
            vec!["w_e", "s_w"],
            vec!["w_e", "e_s"],
            vec!["w_e", "n_s"]
        ];

        for zone in intersection_zones {
            self.add(
                CollideZone::new(
                    zone
                        .iter()
                        .map(|x| x.to_string())
                        .collect()
                )
            );
        }
    }
}
