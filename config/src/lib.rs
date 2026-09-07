/*----------- window config ------------*/
pub const SCREEN_WIDTH: i32 = 800;
pub const SCREEN_HEIGHT: i32 = 600;
pub const CAR_WIDTH: i32 = 30;
pub const CAR_HEIGHT: i32 = 52;
/*----------- lights and sensor config ------------*/
pub const LIGHT_WIDTH: i32 = CAR_WIDTH/4;
pub const LIGHT_HEIGHT: i32 = CAR_HEIGHT;
pub const LIGHT_WIDTH_MARGIN: i32 = CAR_WIDTH/4+CAR_WIDTH/8;
// ----------
pub const HEAD_LIGHT_WIDTH: i32 = CAR_WIDTH/4;
pub const HEAD_LIGHT_HEIGHT: i32 = CAR_HEIGHT;
pub const HEAD_LIGHT_WIDTH_MARGIN: i32 = CAR_WIDTH/4+CAR_WIDTH/8;

pub const WAIT_TIME: u64 = 300;
/*----------- lanes coord config ------------*/
pub const NORTH_L_COORD: (i32, i32) = (SCREEN_WIDTH / 2 - 40, 0);
pub const NORTH_S_COORD: (i32, i32) = (SCREEN_WIDTH / 2 - 47 * 2, 0);
pub const NORTH_R_COORD: (i32, i32) = (SCREEN_WIDTH / 2 - 49 * 3, 0);

pub const SOUTH_R_COORD: (i32, i32) = (SCREEN_WIDTH / 2 + 57 * 2, SCREEN_HEIGHT - 70);
pub const SOUTH_S_COORD: (i32, i32) = (SCREEN_WIDTH / 2 + 62, SCREEN_HEIGHT - 70);
pub const SOUTH_L_COORD: (i32, i32) = (SCREEN_WIDTH / 2 + 18, SCREEN_HEIGHT - 70);

pub const WEST_R_COORD: (i32, i32) = (0, SCREEN_HEIGHT / 2 + 88);
pub const WEST_S_COORD: (i32, i32) = (0, SCREEN_HEIGHT / 2 + 45);
pub const WEST_L_COORD: (i32, i32) = (0, SCREEN_HEIGHT / 2 + 5);

pub const EAST_L_COORD: (i32, i32) = (
    SCREEN_WIDTH - 70,
    SCREEN_HEIGHT / 2 - SCREEN_HEIGHT / 4 + 117,
);
pub const EAST_S_COORD: (i32, i32) = (
    SCREEN_WIDTH - 70,
    SCREEN_HEIGHT / 2 + 71 - SCREEN_HEIGHT / 4,
);
pub const EAST_R_COORD: (i32, i32) = (
    SCREEN_WIDTH - 70,
    SCREEN_HEIGHT / 2 - SCREEN_HEIGHT / 4 + 30,
);

/*----------- intersections coord config ------------*/

pub const WEST_L_INTERSECT: i32 = (SCREEN_WIDTH / 6) * 3 + 10;
pub const WEST_R_INTERSECT: i32 = SCREEN_WIDTH / 6 + 110;

pub const EAST_L_INTERSECT: i32 = (SCREEN_WIDTH / 6) * 4;
pub const EAST_R_INTERSECT: i32 = (SCREEN_WIDTH / 6) * 3 - 30;

pub const NORTH_L_INTERSECT: i32 = SCREEN_HEIGHT / 6 + 70;
pub const NORTH_R_INTERSECT: i32 = (SCREEN_HEIGHT / 6) * 3 + 4;

pub const SOUTH_R_INTERSECT: i32 = (SCREEN_HEIGHT / 6) * 4 - 5;
pub const SOUTH_L_INTERSECT: i32 = (SCREEN_HEIGHT / 6) * 2 + 70;

/*----------- car speed config ------------*/
pub const MAX_SPEED: i32 = 8;
pub const MIN_SPEED: i32 = 3;
pub const SAFE_DISTANCE: i32 = 70;