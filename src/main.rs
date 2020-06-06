/*
Any live cell with two or three live neighbours survives.
Any Dead cell with three live neighbours becomes a live cell.
All other live cells die in the next generation. Similarly, all other Dead cells stay Dead.
*/

use itertools::Itertools;
use std::fmt;
use std::thread;
use std::time;
use rand::Rng;


#[derive(Clone)]
struct Term {
    w: isize,
    h: isize,
}

impl Default for Term {
    fn default() -> Self {
        let (w, h) = term_size::dimensions().expect("Unable to get term dimensions");
        Term {
            w: w as isize - 2,
            h: h as isize - 2,
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.w, self.h)
    }
}

#[derive(PartialEq, Debug, Clone)]
enum CellState {
    Alive,
    Dead,
}

impl Default for CellState {
    fn default() -> CellState {
        CellState::Dead
    }
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            CellState::Alive => "*",
            CellState::Dead => " ",
        };
        write!(f, "{}", c)
    }
}

#[derive(Clone)]
struct LifeBoard {
    iteration: usize,
    cells: Vec<Vec<Cell>>,
    dimensions: Term,
}

impl fmt::Display for LifeBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

impl From<Term> for LifeBoard {
    fn from(t: Term) -> LifeBoard {
        let mut board = LifeBoard {
            cells: vec![vec![Cell::default(); t.w as usize]; t.h as usize],
            dimensions: t,
            iteration: 0,
        };

        board.insert_oscillator(board.get_random_location());
        board.insert_oscillator(board.get_random_location());
        board.insert_glider(board.get_random_location());
        board.insert_glider(board.get_random_location());
        board.assign_locations();
        board
    }
}

impl LifeBoard {

    fn insert_oscillator(&mut self, top_left : CellLocation) -> Option<CellLocation>{
        let thingy_width = 3;
        let thingy_height = 3;

        if top_left.c + thingy_width >= self.dimensions.w
            || top_left.r + thingy_height >= self.dimensions.h
        {
            return None;
        }

        self.cells[top_left.r as usize][top_left.c as usize + 1].state = CellState::Alive;
        self.cells[top_left.r as usize + 1][top_left.c as usize + 1].state = CellState::Alive;
        self.cells[top_left.r as usize + 2][top_left.c as usize + 1].state = CellState::Alive;
        Some(top_left)
    }

    fn get_random_location(&self)-> CellLocation {
        let mut rng = rand::thread_rng();
        CellLocation{r:rng.gen_range(0, self.dimensions.h), c: rng.gen_range(0, self.dimensions.w)}
    }
    
    fn insert_glider(&mut self, top_left: CellLocation) -> Option<CellLocation>{
        let glider_width = 3;
        let glider_height = 3;
        if top_left.c + glider_width >= self.dimensions.w
            || top_left.r + glider_height >= self.dimensions.h
        {
            return None;
        }

        self.cells[top_left.r as usize][top_left.c as usize + 1].state = CellState::Alive;
        self.cells[top_left.r as usize + 1][top_left.c as usize + 2].state = CellState::Alive;
        self.cells[top_left.r as usize + 2][top_left.c as usize].state = CellState::Alive;
        self.cells[top_left.r as usize + 2][top_left.c as usize + 1].state = CellState::Alive;
        self.cells[top_left.r as usize + 2][top_left.c as usize + 2].state = CellState::Alive;
        Some(top_left)
    }
}

#[derive(Debug, Clone)]
struct CellLocation {
    r: isize,
    c: isize,
}

impl Default for CellLocation {
    fn default() -> Self {
        CellLocation { r: 0, c: 0 }
    }
}

impl fmt::Display for CellLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(r {}, c {})", self.r, self.c)
    }
}

#[derive(Clone, Debug)]
struct Cell {
    state: CellState,
    location: CellLocation,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            state: CellState::default(),
            location: CellLocation::default(),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.state)
    }
}

#[derive(Debug)]
enum Direction {
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

impl LifeBoard {
    fn get_relative_cell(&self, from: &Cell, dir: Direction) -> Option<&Cell> {
        let delta = CellLocation::from(dir);
        let next_location = from.location.clone() + delta;
        if next_location.c < 0 || next_location.r < 0 {
            return None;
        }

        if next_location.c >= self.dimensions.w || next_location.r >= self.dimensions.h {
            return None;
        }

        Some(&self.cells[next_location.r as usize][next_location.c as usize])
    }

    fn assign_locations(&mut self) {
        for (r_idx, row) in self.cells.iter_mut().enumerate() {
            for (c_idx, cell) in row.iter_mut().enumerate() {
                cell.location = CellLocation {
                    r: r_idx as isize,
                    c: c_idx as isize,
                }
            }
        }
    }

    fn count_neighbours(&self, c: &Cell) -> u8 {
        let cells = vec![
            self.get_relative_cell(c, Direction::TopLeft),
            self.get_relative_cell(c, Direction::TopRight),
            self.get_relative_cell(c, Direction::TopMiddle),
            self.get_relative_cell(c, Direction::BottomLeft),
            self.get_relative_cell(c, Direction::BottomRight),
            self.get_relative_cell(c, Direction::BottomMiddle),
            self.get_relative_cell(c, Direction::Left),
            self.get_relative_cell(c, Direction::Right),
        ];
        let somes: Vec<&Cell> = cells.into_iter().filter_map(|n| n).collect();
        let alives: Vec<&Cell> = somes
            .into_iter()
            .filter(|n| n.state == CellState::Alive)
            .collect();
        alives.len() as u8
    }

    fn process(self) -> Self {
        let mut next = self.clone();
        for row in next.cells.iter_mut() {
            for cell in row.iter_mut() {
                let neighbours = self.count_neighbours(cell);

                match cell.state {
                    CellState::Alive => {
                        if neighbours != 2 && neighbours != 3 {
                            cell.state = CellState::Dead;
                        }
                    }
                    CellState::Dead => {
                        if neighbours == 3 {
                            cell.state = CellState::Alive;
                        }
                    }
                }
            }
        }
        next
    }
}

fn main() {
    let mut lb = LifeBoard::from(Term::default());
    loop {
        print!("\x1B[2J");
        println!("{}", &lb);
        thread::sleep(time::Duration::from_millis(100));
        lb = lb.process();
    }
}
