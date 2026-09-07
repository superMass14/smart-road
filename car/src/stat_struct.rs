use crate::*;
use sdl2::pixels::Color;
use sdl2::ttf::*;
pub struct Stat {
    pub count: i32,
    pub max_velocity: f64,
    pub min_velocity: f64,
    pub max_time: f64,
    pub min_time: f64,
    pub avoid_accident: i32,
    pub cars: Vec<Car>,
    pub collision: u32,
}

impl Stat {
    pub fn new() -> Self {
        Self {
            count: 0,
            max_velocity: 0.0,
            min_velocity: 0.0,
            max_time: 0.0,
            min_time: 0.0,
            avoid_accident: 0,
            cars: Vec::new(),
            collision: 0,
        }
    }
    pub fn close_call(&mut self) {
        self.avoid_accident += 1;
    }

    pub fn max_vel(&mut self) {
        let mut max_speed = 0.0;
        for car in &self.cars {
            let current_speed = car.distance / car.last_time.as_secs_f64();
            // println!("d => {}, time => {}, speed => {}", car.distance, car.last_time.as_secs_f64(), current_speed);
            if current_speed > max_speed {
                max_speed = current_speed;
            }
        }
        self.max_velocity = max_speed;
        //  println!("final {}", self.max_time);
    }

    pub fn min_vel(&mut self) {
        let mut min_speed = f64::MAX;
        for car in &self.cars {
            let current_speed = car.distance / car.last_time.as_secs_f64();
            if current_speed < min_speed {
                min_speed = current_speed;
            }
        }
        self.min_velocity = if min_speed == f64::MAX { 0.0 } else { min_speed };
    }
    pub fn max_t(&mut self) {
        let selected_car = self.cars
            .clone()
            .into_iter()
            .max_by_key(|car| car.last_time);
        if selected_car.is_some() {
            self.max_time = selected_car.unwrap().last_time.as_secs_f64();
        }
    }

    pub fn min_t(&mut self) {
        let selected_car = self.cars
            .clone()
            .into_iter()
            .min_by_key(|car| car.last_time);
        if selected_car.is_some() {
            self.min_time = selected_car.unwrap().last_time.as_secs_f64();
        }
    }

    pub fn draw(
        &self,
        canvas2: &mut Canvas<Window>,
        context: &Sdl2TtfContext,
        position: &String,
        text: &String,
        line: usize
    ) {
        // Dessiner le texte
        let font_path = Path::new("assets/8-bit-hud.ttf");
        let font = context.load_font(font_path, 10).unwrap();
        let surface = font.render(&text.as_str()).blended(Color::BLACK).unwrap();
        let texture_creator = canvas2.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

        let target = match position.as_str() {
            "left" => Rect::new(40, 30 * ((line as i32) + 1), surface.width(), surface.height()),
            "right" => Rect::new(300, 30 * ((line as i32) + 1), surface.width(), surface.height()),
            "center" => Rect::new(185, 235, surface.width(), surface.height()),
            _ => panic!("wrong target (fn draw stat)"),
        };
        canvas2.copy(&texture, None, Some(target)).unwrap();
    }
    pub fn render(&mut self, canvas2: &mut Canvas<Window>, context: &Sdl2TtfContext) {
        self.max_t();
        self.min_t();
        self.max_vel();
        self.min_vel();
        // let mut line = 0;
        let names = vec![
            "Max number of vehicles:",
            "Max_velocity:",
            "Min_velocity:",
            "Max_time:",
            "Min_time:",
            "close calls:",
            "collision:"
        ];
        canvas2.set_draw_color(Color::RGB(208, 200, 112));
        canvas2.clear();

        canvas2.set_draw_color(Color::RGB(0, 200, 0));
        let _ = canvas2.fill_rect(Rect::new(150, 220, 100, 50));
        for i in 0..7 {
            self.draw(canvas2, context, &"left".to_string(), &names[i].to_string(), i);
        }
        self.draw(canvas2, context, &"right".to_string(), &self.cars.len().to_string(), 0);
        self.draw(canvas2, context, &"right".to_string(), &format!("{:.2}", self.max_velocity), 1);
        self.draw(canvas2, context, &"right".to_string(), &format!("{:.2}", self.min_velocity), 2);
        self.draw(canvas2, context, &"right".to_string(), &format!("{:.2}", self.max_time), 3);
        self.draw(canvas2, context, &"right".to_string(), &format!("{:.2}", self.min_time), 4);
        self.draw(canvas2, context, &"right".to_string(), &format!("{}", self.avoid_accident), 5);
        self.draw(canvas2, context, &"right".to_string(), &format!("{}", self.collision), 6);

        self.draw(canvas2, context, &"center".to_string(), &"Ok".to_string(), 0);
        canvas2.present();
    }
}
