use crate::*;


pub fn get_intersect_head_lights(zone:Rect,car:&Car) -> Vec<Rect> {
    let mut result = Vec::new();
    if car.x < zone.x && car.x+car.width as i32 > zone.x+zone.w {
        result.push(Rect::new(zone.x -zone.w, zone.y, zone.w as u32, zone.h as u32));
        result.push(Rect::new(zone.x +zone.w, zone.y, zone.w as u32, zone.h as u32));
    }
    result
}
pub fn get_car_rect(car:&Car) -> Rect {
    match car.angle {
        0 => Rect::new(car.x-3, car.y+4, car.width-6, car.height-8),
        180 => Rect::new(car.x+4, car.y+4, car.width+2, car.height-8),
        90 => Rect::new(car.x+3, car.y-2, car.width-6, car.height-5),
        270 => Rect::new(car.x+4, car.y+4, car.width-7, car.height-2),
        _ => Rect::new(car.x, car.y, car.width, car.height)
    }
}
pub fn control_trafic(
    vehicles: &mut Vec<Car>,
    _canvas: &mut Canvas<Window>,
    collide_zones: &CollideZones,
    stat: &mut Stat
) {
    let clone = vehicles.clone();
    let mut priority_cars = Vec::new();
    let mut stoped_cars = Vec::new();
    for (i, car) in vehicles.iter_mut().enumerate() {
        // verifier avant d'entrer dans la zone des intersections
        let intersections_area = Rect::new(300, 200, 200, 200);
        let intersections_area_container = match car.from {
            Location::West =>Rect::new(200, 300, 300, 150),
            Location::East =>Rect::new(300, 150, 300, 150),
            Location::North =>Rect::new(250, 100, 150, 300),
            Location::South =>Rect::new(400, 200, 150, 300),
        };
        // _canvas.set_draw_color(Color::MAGENTA);
        // let _ = _canvas.fill_rect(intersections_area_container);
        // _canvas.set_draw_color(Color::BLACK);
        // let _ = _canvas.fill_rect(intersections_area);

        let mut left_cars = 0;

        let mut should_collide = false;
        let mut has_itersect = false;
        if !priority_cars.contains(&i) {
            for (j, new_car) in clone.iter().enumerate() {
                if car != new_car {
                    for hls in car.head_lights_stop.clone() {
                        if hls.has_intersection(get_car_rect(new_car))
                        {
                            stoped_cars.push(j)
                        }
                    }
                    // --------pour eviter qu'il y est trois voitures dans l'intersection
                    if 
                        get_car_rect(car).has_intersection(intersections_area_container) &&
                        car.direction == Direction::Left
                    {
                        if 
                            get_car_rect(new_car).has_intersection(intersections_area) &&
                            new_car.direction == Direction::Left
                        {
                            left_cars += 1;
                        }
                    }
                    // ------------------collide_zones
                    for zone in collide_zones.0.iter() {
                        if
                            car.light.has_intersection(zone.collide)
                        {
                            for rect in get_intersect_head_lights(zone.collide, car) {
                                // _canvas.set_draw_color(Color::BLACK);
                                // let _ = _canvas.fill_rect(rect);
                                if rect.has_intersection(
                                    get_car_rect(new_car)
                                ) {
                                    let mut can_circulate = true;
                                    let mut light_interseect = false;
                                    for c in clone.iter() {
                                        if c != car
                                        {
                                            if car.light.has_intersection(get_car_rect(c)) {
                                                light_interseect = true
                                            }
                                            if rect.has_intersection(get_car_rect(c)) {
                                                can_circulate = false;
                                            }
                                        }
                                    }
                                    if !light_interseect  {
                                        break;
                                    }
                                    if can_circulate {
                                    }else {
                                        should_collide = true;
                                        break;
                                    }
                                }
                           } 
                        }
                        // ------------------
                        if
                            car.light.has_intersection(zone.collide) &&
                            zone.collide.has_intersection(
                                get_car_rect(new_car)
                            )
                        {
                            should_collide = true;
                            if
                                car.velocity_x == 0 &&
                                car.velocity_y == 0 &&
                                new_car.velocity_x == 0 &&
                                new_car.velocity_y == 0
                            {
                                priority_cars.push(j);
                            }
                            // std::thread::sleep(Duration::from_millis(1000));
                            break;
                        }
                    }
                    // -------------------
                    if
                        get_car_rect(new_car).has_intersection(get_car_rect(car)) &&
                        new_car.x == car.x && car.y == new_car.y             
                    {
                        println!("oops collide");
                        stat.collision += 1;    
                    }
                    if
                        car.light.has_intersection(
                            get_car_rect(new_car)
                        )
                    {
                        has_itersect = true;
                        if
                            car.sensor.has_intersection(
                                get_car_rect(new_car)
                            )
                        {
                            should_collide = true;
                        }
                        // _canvas.set_draw_color(Color::WHITE);
                        // let _ = _canvas.fill_rect(get_car_rect(car));
                        // if car.angle == 0 {
                        //     // std::thread::sleep(Duration::from_millis(3000));
                        // }
                    }
                }
            }
        }

        if stoped_cars.contains(&i) {
            car.stop()
        }else if 
            should_collide || 
            left_cars >= 2 && !get_car_rect(car).has_intersection(intersections_area)
        {
            car.stop();
            if !car.has_collision {
                car.has_collision = true;
                stat.close_call();
            }
        } else if has_itersect {
            car.throttle();
        } else {
            car.accelerate();
            car.has_collision = false;
        }
    }
}
