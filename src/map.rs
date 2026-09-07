use sdl2::pixels::Color;
use sdl2::render::*;
use sdl2::video::*;
use std::path::Path;
use sdl2::rect::*;
use sdl2::image::LoadTexture;
use config::*;

pub fn draw_map(canvas: &mut Canvas<Window>) {
    let texture_innit = canvas.texture_creator();
    let path = Path::new("assets/Road.png");
    let texture = texture_innit.load_texture(path).unwrap();
    let car_form = Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    canvas.copy(&texture, None, Some(car_form)).unwrap();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
}
