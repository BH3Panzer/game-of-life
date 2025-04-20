use raylib::prelude::*;
use crate::CELL_SIZE;

#[derive(Clone)]
pub struct Cell {
    alive: bool,
    alive_next: bool,
    x: i32,
    y: i32,
    alive_neighbours: u8
}

impl Cell {
    pub fn new(x: i32, y: i32, alive: bool) -> Cell {
        Cell {
            alive: alive,
            alive_next: alive,
            x,
            y,
            alive_neighbours: 0
        }
    }

    pub fn draw(&mut self, d: &mut raylib::core::drawing::RaylibDrawHandle) {
        let mut color : Color = Color::RAYWHITE;
        let mouse_pos = d.get_mouse_position();
        
        if self.alive {
            color = Color::BLACK;
        }
        // Check if the mouse is over the cell
        if mouse_pos.x >= self.x as f32 * CELL_SIZE as f32 && mouse_pos.x < self.x as f32 * CELL_SIZE as f32 + CELL_SIZE as f32 && mouse_pos.y >= self.y as f32 * CELL_SIZE as f32 && mouse_pos.y < self.y as f32 * CELL_SIZE as f32 + CELL_SIZE as f32 {
            color = Color::BLUE;
            if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                self.alive = true;
                color = Color::LIGHTGREEN;
            } else if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
                self.alive = false;
                color = Color::ORANGE;
            }
        }
        if self.x*CELL_SIZE <= d.get_screen_width() as i32 && self.y*CELL_SIZE <= d.get_screen_height() as i32 {
            d.draw_rectangle(self.x * CELL_SIZE, self.y * CELL_SIZE, CELL_SIZE, CELL_SIZE, color);
        }
        
        
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn set_alive(&mut self, alive: bool) {
        self.alive_next = alive;
    }

    pub fn flush_alive(&mut self) {
        self.alive = self.alive_next;
    }

    pub fn set_alive_neighbours(&mut self, alive_neighbours: u8) {
        self.alive_neighbours = alive_neighbours;
    }

    pub fn get_alive_neighbours(&self) -> u8 {
        self.alive_neighbours
    }
}
