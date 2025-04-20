
use crate::{cell::Cell, GRID_SIZE};

pub struct Grid {
    pub cells: Vec<Cell>,
    number_of_alive_cells: u32,
    number_of_dead_cells: u32,
    number_of_cells: u32
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            cells: Vec::new(),
            number_of_alive_cells: 0,
            number_of_dead_cells: 0,
            number_of_cells: 0
        }
    }
    pub fn fill_with_cells(&mut self, size: i32, alives: bool) {
        self.cells.clear();
        for x in 0..size {
            for y in 0..size {
                self.cells.push(Cell::new(x, y, alives));
            }
        }
    }


    pub fn draw(&mut self, d: &mut raylib::core::drawing::RaylibDrawHandle) {
        for cell in self.cells.iter_mut() {
            cell.draw(d);
        }
    }

    pub fn update_number_of_cells(&mut self) {
        self.number_of_alive_cells = 0;
        self.number_of_dead_cells = 0;
        self.number_of_cells = 0;
        for cell in self.cells.iter_mut() {
            if cell.is_alive() {
                self.number_of_alive_cells += 1;
            } else {
                self.number_of_dead_cells += 1;
            }
        }
        self.number_of_cells = self.number_of_alive_cells + self.number_of_dead_cells;

    }

    pub fn get_number_of_alive_cells(&self) -> u32 {
        self.number_of_alive_cells
    }

    pub fn get_number_of_dead_cells(&self) -> u32 {
        self.number_of_dead_cells
    }

    pub fn get_number_of_cells(&self) -> u32 {
        self.number_of_cells
    }

    pub fn calculate_next_iteration(&mut self) {
        let old_cells = self.cells.clone();
        let mut next_states = Vec::with_capacity(old_cells.len());
        for (i, cell) in self.cells.iter_mut().enumerate() {
            let mut alive_neighbours = 0;

            let x = i % GRID_SIZE as usize;
            let y = i / GRID_SIZE as usize;

            let deltas = [
                (-1, -1), (0, -1), (1, -1),
                (-1,  0),          (1,  0),
                (-1,  1), (0,  1), (1,  1),
            ];
            
            for (dx, dy) in deltas {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
        
                if nx >= 0 && ny >= 0 && (nx as usize) < GRID_SIZE as usize && (ny as usize) < GRID_SIZE as usize {
                    let n_index = ny as usize * GRID_SIZE as usize + nx as usize;
                    if old_cells[n_index].is_alive() {
                        alive_neighbours += 1;
                    }
                }
            }

            cell.set_alive_neighbours(alive_neighbours);

            let new_state = if cell.get_alive_neighbours() < 2 || cell.get_alive_neighbours() > 3 {
                false
            } else if cell.get_alive_neighbours() == 3 {
                true
            } else {
                cell.is_alive()
            };

            next_states.push(new_state);

            cell.set_alive(new_state);

        }
    }
}