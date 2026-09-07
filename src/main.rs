mod map;

use car::Stat;
use car::{ car_struct::Car, control_trafic, CollideZones };
use event::{ handle_event, Throttle };
use map::*;
use config::*;
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("SMART ROAD", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut display_stats = false;
    let mut stats_canvas: Option<sdl2::render::Canvas<sdl2::video::Window>> = None;
    let mut stat: Stat = Stat::new();

    let mut vehicles: Vec<Car> = Vec::new();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut throttle = Throttle::new();
    let mut collide_zones = CollideZones::new();
    collide_zones.init();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        draw_map(&mut canvas);

        // collide_zones.draw_collide_zone(&mut canvas);

        if
            handle_event(
                &mut event_pump,
                &mut vehicles,
                &mut collide_zones,
                &mut throttle,
                &video_subsystem,
                &mut stats_canvas,
                &mut display_stats
            ).is_err()
        {
            break 'running;
        }

        vehicles.retain(|v| !v.arrived);
        control_trafic(&mut vehicles, &mut canvas, &collide_zones, &mut stat);
        let vehicles_clone = vehicles.clone();
        for vehicle in vehicles.iter_mut() {
            // vehicle.accelerate();
            if !vehicle.is_blocked {
                vehicle.head_lights = Vec::new();
                vehicle.head_lights_stop = Vec::new();
            }
            vehicle.vroom(vehicles_clone.clone(), &mut stat);
            vehicle.draw(&mut canvas);
        }
        // Dessiner dans la deuxième fenêtre
        if let Some(ref mut canvas2) = stats_canvas {
            stat.render(canvas2, &ttf_context);
        }
        canvas.present();
        // std::thread::sleep(Duration::from_millis(500))
    }
}
