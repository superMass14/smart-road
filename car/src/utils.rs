use rand::*;
use crate::car_struct::*;
use config::*;

pub fn get_destination(spawn: Location, direction: Direction) -> Location {
    match (spawn, direction) {
        | (Location::North, Direction::Right)
        | (Location::South, Direction::Left)
        | (Location::East, Direction::Straight) => Location::West,

        | (Location::North, Direction::Left)
        | (Location::South, Direction::Right)
        | (Location::West, Direction::Straight) => Location::East,

        | (Location::North, Direction::Straight)
        | (Location::West, Direction::Right)
        | (Location::East, Direction::Left) => Location::South,

        | (Location::South, Direction::Straight)
        | (Location::West, Direction::Left)
        | (Location::East, Direction::Right) => Location::North,
    }
}

pub fn get_distance(from: Location, to: Location) -> f64 {
    match (from, to) {
        | (Location::North, Location::West)
        | (Location::East, Location::North)
        | (Location::South, Location::East)
        | (Location::West, Location::South) => 471.0,

        (Location::North, Location::South) | (Location::South, Location::North) =>
            SCREEN_HEIGHT as f64,

        (Location::West, Location::East) | (Location::East, Location::West) => SCREEN_WIDTH as f64,

        | (Location::North, Location::East)
        | (Location::South, Location::West)
        | (Location::West, Location::North)
        | (Location::East, Location::South) => 746.0,
        _ => panic!("wrong distance"),
    }
}

pub fn random_spawn() -> Location {
    let mut gen = thread_rng();
    let s_index = gen.gen_range(1, 5);
    let location = match s_index {
        1 => Location::North,
        2 => Location::South,
        3 => Location::East,
        _ => Location::West,
    };

    location
}

pub fn random_direction() -> Direction {
    let mut gen = thread_rng();
    let d_index = gen.gen_range(1, 6);
    //let d_index = 3;
    // println!("rand {}", d_index);
    let direction = match d_index {
        1|2 => Direction::Straight,
        3|4 => Direction::Right,
        _ => Direction::Left,
    };
    //println!("rand {} {:?}", d_index, direction);

    direction
    //Direction::Left
}
pub fn get_coords(spawn: Location, direction: Direction) -> (i32, i32) {
    match (spawn, direction) {
        (Location::North, Direction::Left) => NORTH_L_COORD,
        (Location::North, Direction::Right) => NORTH_R_COORD,
        (Location::North, Direction::Straight) => NORTH_S_COORD,

        (Location::South, Direction::Left) => SOUTH_L_COORD,
        (Location::South, Direction::Right) => SOUTH_R_COORD,
        (Location::South, Direction::Straight) => SOUTH_S_COORD,

        (Location::West, Direction::Left) => WEST_L_COORD,
        (Location::West, Direction::Right) => WEST_R_COORD,
        (Location::West, Direction::Straight) => WEST_S_COORD,

        (Location::East, Direction::Left) => EAST_L_COORD,
        (Location::East, Direction::Right) => EAST_R_COORD,
        (Location::East, Direction::Straight) => EAST_S_COORD,
    }
}

pub fn can_move(new_car: &Car, cars: &[Car]) -> bool {
    if cars.len() < 12 {
        for car in cars {
            if
                (new_car.from == Location::South &&
                    car.from == Location::South &&
                    car.y + CAR_HEIGHT + SAFE_DISTANCE >= new_car.y) ||
                (new_car.from == Location::North &&
                    car.from == Location::North &&
                    car.y <= new_car.y + CAR_HEIGHT + SAFE_DISTANCE) ||
                (new_car.from == Location::East &&
                    car.from == Location::East &&
                    car.x + CAR_WIDTH + SAFE_DISTANCE >= new_car.x) ||
                (new_car.from == Location::West &&
                    car.from == Location::West &&
                    car.x <= new_car.x + CAR_WIDTH + SAFE_DISTANCE)
            {
                return false;
            }
        }
        return true;
    }
    false
}

pub fn has_safety_d(car: &Car, next_car: &Car) -> bool {
    car.x.abs() + (car.width as i32) - next_car.x.abs() + (car.width as i32) >= SAFE_DISTANCE ||
        car.y.abs() + (car.height as i32) - next_car.y.abs() + (car.height as i32) >= SAFE_DISTANCE
}

pub fn format_location(from: Location, to: Location) -> String {
    match (from, to) {
        (Location::North, Location::East) => "n_e".to_string(),
        (Location::North, Location::West) => "n_w".to_string(),
        (Location::North, Location::South) => "n_s".to_string(),
        (Location::South, Location::East) => "s_e".to_string(),
        (Location::South, Location::West) => "s_w".to_string(),
        (Location::South, Location::North) => "s_n".to_string(),
        (Location::East, Location::West) => "e_w".to_string(),
        (Location::East, Location::North) => "e_n".to_string(),
        (Location::East, Location::South) => "e_s".to_string(),
        (Location::West, Location::South) => "w_s".to_string(),
        (Location::West, Location::North) => "w_n".to_string(),
        (Location::West, Location::East) => "w_e".to_string(),
        _ => { panic!("wrong format") }
    }
}
