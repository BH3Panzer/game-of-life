use raylib::prelude::*;
mod cell;
mod grid;
mod kamera;

const GRID_SIZE: i32 = 250;

const CELL_SIZE: i32 = 2;


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(0, 0)
        .title("Game of Life")
        .build();
    
    rl.toggle_fullscreen();
    let mut camera: kamera::Kamera = kamera::Kamera::new();
    camera.set_position(-rl.get_screen_width() as f32 / 3.0, -rl.get_screen_height() as f32 / 3.0);
    let mut grid: grid::Grid = grid::Grid::new();
    let mut frame: u32 = 0;
    let mut iteration: u32 = 0;
    let mut paused: bool = true;
    let mut speed: u8 = 1;
    let mut dt: f32;
    grid.fill_with_cells(GRID_SIZE, false);
    while !rl.window_should_close() {
        frame += 1;
        dt = rl.get_frame_time();
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            speed = 50;
        } else if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            speed = 25;
        } else if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            speed = 10;
        } else if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
            speed = 5;
        } else if rl.is_key_pressed(KeyboardKey::KEY_FIVE) {
            speed = 1;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            camera.translate(0 as f32 * dt, -500 as f32 *dt);
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            camera.translate(0 as f32 * dt, 500 as f32 * dt);
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            camera.translate(-500 as f32 * dt, 0 as f32 * dt);
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            camera.translate(500 as f32 * dt, 0 as f32 * dt);
        }
        if rl.get_mouse_wheel_move() > 0.0 {
            camera.change_zoom(1.0);
            camera.translate(rl.get_mouse_position().x as f32 / 3.0, rl.get_mouse_position().y as f32 / 3.0);
        } else if rl.get_mouse_wheel_move() < 0.0 {
            if camera.get_zoom() > 1.0 {
                camera.change_zoom(-1.0);
                camera.translate(-rl.get_mouse_position().x as f32 / 3.0, -rl.get_mouse_position().y as f32 / 3.0);
            }
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_MIDDLE) {
            camera.set_position(-rl.get_screen_width() as f32 / 3.0, -rl.get_screen_height() as f32 / 3.0);
        }


        if !paused {
            if frame % speed as u32 == 0 {
                grid.calculate_next_iteration();
                for cell in grid.cells.iter_mut() {
                    cell.flush_alive();
                }
                iteration += 1;
            }
        }


        if rl.is_key_pressed(KeyboardKey::KEY_DELETE) {
            grid.fill_with_cells(GRID_SIZE, false);
            iteration = 0;
            frame = 0;
            paused = true;
        }
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::GHOSTWHITE);

        grid.draw(&mut d, camera.get_x(), camera.get_y(), camera.get_zoom());

        d.draw_fps(0, 0);

        grid.update_number_of_cells();

        d.draw_text(&format!("alive cells: {}", grid.get_number_of_alive_cells()), 5, 20, 20, Color::DARKGREEN);
        d.draw_text(&format!("dead cells: {}", grid.get_number_of_dead_cells()), 5, 40, 20, Color::RED);
        d.draw_text(&format!("cells: {}", grid.get_number_of_cells()), 5, 60, 20, Color::BLACK);
        d.draw_text(&format!("frame: {}", frame), 5, 80, 20, Color::BLACK);
        d.draw_text(&format!("iteration: {}", iteration), 5, 100, 20, Color::BLACK);
        d.draw_text(&format!("speed: {}", speed), 5, 120, 20, Color::BLACK);
    }
}
