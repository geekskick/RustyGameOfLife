use super::cells::*;
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Term {
    pub w: i32,
    pub h: i32,
}

impl Default for Term {
    fn default() -> Self {
        let (w, h) = term_size::dimensions().expect("Unable to get term dimensions");
        Term {
            w: (w - 2) as i32,
            h: (h - 2) as i32,
        }
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.w, self.h)
    }
}

#[derive(Clone)]
pub struct LifeBoard {
    iteration: usize,
    pub cells: Vec<Vec<Cell>>,
    dimensions: Term,
}

impl std::fmt::Display for LifeBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
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

        board.assign_locations();
        board
    }
}

impl LifeBoard {
    #[allow(unused)]
    fn insert_oscillator(&mut self, top_left: CellLocation) -> Option<CellLocation> {
        let thingy_width = 3;
        let thingy_height = 3;

        if top_left.c + thingy_width >= self.dimensions.w || top_left.r + thingy_height >= self.dimensions.h {
            return None;
        }

        self.cells[top_left.r as usize][top_left.c as usize + 1].state = CellState::Alive;
        self.cells[top_left.r as usize + 1][top_left.c as usize + 1].state = CellState::Alive;
        self.cells[top_left.r as usize + 2][top_left.c as usize + 1].state = CellState::Alive;
        Some(top_left)
    }
    #[allow(unused)]
    fn get_random_location(&self) -> CellLocation {
        let mut rng = rand::thread_rng();
        CellLocation {
            r: rng.gen_range(0, self.dimensions.h),
            c: rng.gen_range(0, self.dimensions.w),
        }
    }
    #[allow(unused)]
    fn insert_glider(&mut self, top_left: CellLocation) -> Option<CellLocation> {
        let glider_width = 3;
        let glider_height = 3;
        if top_left.c + glider_width >= self.dimensions.w || top_left.r + glider_height >= self.dimensions.h {
            return None;
        }

        self.cells[top_left.r as usize][top_left.c as usize + 1].state = CellState::Alive;
        self.cells[top_left.r as usize + 1][top_left.c as usize + 2].state = CellState::Alive;
        self.cells[top_left.r as usize + 2][top_left.c as usize].state = CellState::Alive;
        self.cells[top_left.r as usize + 2][top_left.c as usize + 1].state = CellState::Alive;
        self.cells[top_left.r as usize + 2][top_left.c as usize + 2].state = CellState::Alive;
        Some(top_left)
    }

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
                    r: r_idx as i32,
                    c: c_idx as i32,
                };

                cell.state = thread_rng().gen();
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
        let alives: Vec<&Cell> = somes.into_iter().filter(|n| n.state == CellState::Alive).collect();
        alives.len() as u8
    }

    pub fn process(self) -> Self {
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
