use bracket_lib::prelude::RandomNumberGenerator;

/*
 *
 * directions.rs
 * -------------
 * Making directions constants so it'll be easier to manage player movement etc.
 *
 */

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Direction {
    pub delta_x: i8,
    pub delta_y: i8,
}

pub const NONE: Direction = Direction {
    delta_x: 0,
    delta_y: 0,
};

pub const EAST: Direction = Direction {
    delta_x: 1,
    delta_y: 0,
};
pub const WEST: Direction = Direction {
    delta_x: -1,
    delta_y: 0,
};
pub const NORTH: Direction = Direction {
    delta_x: 0,
    delta_y: -1,
};
pub const SOUTH: Direction = Direction {
    delta_x: 0,
    delta_y: 1,
};
pub const NORTHEAST: Direction = Direction {
    delta_x: 1,
    delta_y: -1,
};
pub const NORTHWEST: Direction = Direction {
    delta_x: -1,
    delta_y: -1,
};
pub const SOUTHEAST: Direction = Direction {
    delta_x: 1,
    delta_y: 1,
};
pub const SOUTHWEST: Direction = Direction {
    delta_x: -1,
    delta_y: 1,
};

#[allow(dead_code)]
pub fn opposite(dir: Direction) -> Direction {
    match dir {
        EAST => return WEST,
        WEST => return EAST,
        NORTH => return SOUTH,
        SOUTH => return NORTH,
        NORTHEAST => return SOUTHWEST,
        NORTHWEST => return SOUTHEAST,
        SOUTHEAST => return NORTHWEST,
        _ => return NORTHEAST,
    }
}

#[allow(dead_code)]
pub fn opposite_idx(idx: usize) -> usize {
    match idx {
        0 => return 1, // EAST -> WEST
        1 => return 0, // WEST -> EAST
        2 => return 3, // NORTH -> SOUTH
        3 => return 2, // SOUTH -> NORTH
        4 => return 5, // NORTHEAST -> NORTHWEST
        5 => return 4, // NORTHWEST -> NORTHEAST
        6 => return 7, // SOUTHEAST -> SOUTHWEST
        _ => return 6, // SOUTHWEST -> SOUTHEAST
    }
}

#[allow(dead_code)]
pub fn dir_idx(idx: usize) -> Direction {
    match idx {
        0 => return EAST,
        1 => return WEST,
        2 => return NORTH,
        3 => return SOUTH,
        4 => return NORTHEAST,
        5 => return NORTHWEST,
        6 => return SOUTHEAST,
        _ => return SOUTHWEST,
    }
}

#[allow(dead_code)]
pub fn get_random_dir() -> Direction {
    let mut rng = RandomNumberGenerator::new();
    let dir = rng.range(0, 4);

    match dir {
        0 => {
            return EAST;
        }
        1 => {
            return WEST;
        }
        2 => {
            return NORTH;
        }
        _ => {
            return SOUTH;
        }
    }
}
