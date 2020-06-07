/*
Any live cell with two or three live neighbours survives.
Any Dead cell with three live neighbours becomes a live cell.
All other live cells die in the next generation. Similarly, all other Dead cells stay Dead.
*/

use std::thread;
use std::{collections, time};

mod cells;
mod life;
use cells::*;
use life::*;

struct GUISettings {
    history_length: usize,
    cell_edge: i32,
    board_height: i32,
    board_width: i32,
}

impl Default for GUISettings {
    fn default() -> Self {
        GUISettings {
            history_length: 4,
            cell_edge: 10,
            board_height: 50,
            board_width: 100,
        }
    }
}

fn show_cell(r: &mut sdl2::render::Canvas<sdl2::video::Window>, cell: &Cell, intensity: f32, settings: &GUISettings) {
    let tl_x = cell.location.c * settings.cell_edge;
    let tl_y = cell.location.r * settings.cell_edge;
    let colour_intensity = (255.0 * intensity) as u8;
    let g_b_intensity = if colour_intensity != 255 { colour_intensity } else { 0 };
    let colour = match cell.state {
        CellState::Alive => sdl2::pixels::Color::RGB(colour_intensity, g_b_intensity, g_b_intensity),
        CellState::Dead => return,
    };

    r.set_draw_color(colour);
    r.fill_rect(sdl2::rect::Rect::new(
        tl_x,
        tl_y,
        settings.cell_edge as u32,
        settings.cell_edge as u32,
    ))
    .expect("Unable to draw rectangle");
}

fn main() {
    let settings = GUISettings::default();

    let sdl_context = sdl2::init().expect("Unable to initialise SDL2");

    let vss = sdl_context.video().expect("Unable to get a video context from SDL");

    let window = vss
        .window(
            "Game of Life",
            (settings.board_width * settings.cell_edge) as u32,
            (settings.board_height * settings.cell_edge) as u32,
        )
        .position_centered()
        .build()
        .expect("Unable to create window");
    let mut canvas = window.into_canvas().build().expect("Unable to get a canvas");
    canvas.set_draw_color(sdl2::pixels::Color::BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().expect("Unable to get SDL event pump");

    let mut lb = LifeBoard::from(Term {
        w: settings.board_width,
        h: settings.board_height,
    });

    let mut history = collections::VecDeque::new();
    history.push_front(lb.clone());

    'running: loop {
        canvas.set_draw_color(sdl2::pixels::Color::BLACK);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let age_incr = 1.0 / (history.len() as f32);
        let mut age = age_incr;
        for board in &history {
            for row in &board.cells {
                for cell in row {
                    show_cell(&mut canvas, &cell, age, &settings);
                }
            }
            age += age_incr;
        }

        canvas.present();
        lb = lb.process();

        history.push_back(lb.clone());
        if history.len() > settings.history_length {
            history.pop_front();
        }

        thread::sleep(time::Duration::from_millis(50));
    }
}
