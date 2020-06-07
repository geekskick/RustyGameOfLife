use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
#[derive(PartialEq, Debug, Clone)]
pub enum CellState {
    Alive,
    Dead,
}

impl Default for CellState {
    fn default() -> CellState {
        CellState::Dead
    }
}

impl std::fmt::Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self {
            CellState::Alive => "*",
            CellState::Dead => " ",
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone)]
pub struct CellLocation {
    pub r: i32,
    pub c: i32,
}

impl Default for CellLocation {
    fn default() -> Self {
        CellLocation { r: 0, c: 0 }
    }
}

impl Distribution<CellState> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CellState {
        match rng.gen_range(0, 2) {
            0 => CellState::Alive,
            1 => CellState::Dead,
            _ => panic!("WTF random number"),
        }
    }
}

impl std::fmt::Display for CellLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(r {}, c {})", self.r, self.c)
    }
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub state: CellState,
    pub location: CellLocation,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            state: CellState::default(),
            location: CellLocation::default(),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.state)
    }
}

#[derive(Debug)]
pub enum Direction {
    TopLeft,
    TopMiddle,
    TopRight,
    Left,
    Right,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}

impl std::ops::Add for &CellLocation {
    type Output = CellLocation;

    fn add(self, other: Self) -> CellLocation {
        CellLocation {
            r: self.r + other.r,
            c: self.c + other.c,
        }
    }
}

impl std::ops::Add for CellLocation {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        CellLocation {
            r: self.r + other.r,
            c: self.c + other.c,
        }
    }
}

impl From<Direction> for CellLocation {
    fn from(dir: Direction) -> CellLocation {
        match dir {
            Direction::TopLeft => CellLocation { r: -1, c: -1 },
            Direction::TopMiddle => CellLocation { r: -1, c: 0 },
            Direction::TopRight => CellLocation { r: -1, c: 1 },
            Direction::Left => CellLocation { r: 0, c: -1 },
            Direction::Right => CellLocation { r: 0, c: 1 },
            Direction::BottomLeft => CellLocation { r: 1, c: -1 },
            Direction::BottomMiddle => CellLocation { r: 1, c: 0 },
            Direction::BottomRight => CellLocation { r: 1, c: 1 },
        }
    }
}
