/*
Any live cell with two or three live neighbours survives.
Any Dead cell with three live neighbours becomes a live cell.
All other live cells die in the next generation. Similarly, all other Dead cells stay Dead.
*/

use std::fmt;
use std::thread;
use std::{collections, time};

mod cells;
mod life;
use cells::*;

use clap::{App, Arg};
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

impl fmt::Display for GUISettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "History Length = {}, Cell Edge = {}, Board Height = {}, Board Width = {}",
            self.history_length, self.cell_edge, self.board_height, self.board_width
        )
    }
}

fn show_cell(
    r: &mut sdl2::render::Canvas<sdl2::video::Window>,
    cell: &Cell,
    intensity: f32,
    settings: &GUISettings,
) {
    let tl_x = cell.location.c * settings.cell_edge;
    let tl_y = cell.location.r * settings.cell_edge;
    let colour_intensity = (255.0 * intensity) as u8;
    let g_b_intensity = if colour_intensity != 255 {
        colour_intensity
    } else {
        0
    };
    let colour = match cell.state {
        CellState::Alive => {
            sdl2::pixels::Color::RGB(colour_intensity, g_b_intensity, g_b_intensity)
        }
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

fn check_limits<T: std::fmt::Display + std::cmp::PartialOrd + std::str::FromStr>(
    val: &str,
    lowest_acceptable: T,
    highest_acceptable: T,
) -> Result<(), String> {
    let num = val.parse::<T>();
    let num = match num {
        Err(_) => return Err(format!("Unable to convert {} to a number", val)),
        Ok(num) => num,
    };
    if num < lowest_acceptable || num > highest_acceptable {
        return Err(format!(
            "Number must be >= {} and <= {}",
            lowest_acceptable, highest_acceptable
        ));
    }
    Ok(())
}

struct Logger {
    is_verbose: bool,
}

impl Logger {
    fn nice_to_know(&self, msg: &str) {
        if self.is_verbose {
            println!("{}", msg);
        }
    }
}

fn main() {
    let matches = App::new("Game of Life")
        .version("1.0")
        .author("Patrick Mintram")
        .about("Conways Game Of Life")
        .arg(
            Arg::with_name("history length")
                .takes_value(true)
                .long("history")
                .help("How much fade behind the current shape")
                .validator(|val| check_limits(&val, 1, 20)),
        )
        .arg(
            Arg::with_name("cell edge length")
                .takes_value(true)
                .long("edge")
                .help("How many pixels each cell edge is")
                .validator(|val| check_limits(&val, 1, 20)),
        )
        .arg(
            Arg::with_name("board height")
                .takes_value(true)
                .long("bheight")
                .help("How many cells tall the screen is")
                .validator(|val| check_limits(&val, 10, 200)),
        )
        .arg(
            Arg::with_name("board width")
                .takes_value(true)
                .long("bwidth")
                .help("How many cells wide the screen is")
                .validator(|val| check_limits(&val, 10, 200)),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("enable verbose logging"),
        )
        .get_matches();

    let history_length = clap::value_t!(matches.value_of("history length"), usize).unwrap_or(4);
    let cell_edge = clap::value_t!(matches.value_of("cell edge length"), i32).unwrap_or(10);
    let board_height = clap::value_t!(matches.value_of("board height"), i32).unwrap_or(80);
    let board_width = clap::value_t!(matches.value_of("board width"), i32).unwrap_or(100);
    let is_verbose = matches.is_present("verbose");

    let clog = Logger { is_verbose };

    let settings = GUISettings {
        history_length,
        cell_edge,
        board_width,
        board_height,
    };
    clog.nice_to_know(&format!("Settings\t{}", &settings));

    let sdl_context = sdl2::init().expect("Unable to initialise SDL2");
    clog.nice_to_know("Initialised SDL2");
    clog.nice_to_know(&format!("SDL2 Version = {}", sdl2::version::version()));

    let vss = sdl_context
        .video()
        .expect("Unable to get a video context from SDL");
    clog.nice_to_know("Initialised a video context");

    let window = vss
        .window(
            "Game of Life",
            (settings.board_width * settings.cell_edge) as u32,
            (settings.board_height * settings.cell_edge) as u32,
        )
        .position_centered()
        .build()
        .expect("Unable to create window");
    clog.nice_to_know("Created a window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Unable to get a canvas");
    clog.nice_to_know("Created a canvas");

    canvas.set_draw_color(sdl2::pixels::Color::BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Unable to get SDL event pump");
    clog.nice_to_know("Created an event pump");

    let mut lb = LifeBoard::from(Term {
        w: settings.board_width,
        h: settings.board_height,
    });
    clog.nice_to_know("Created first board");

    let mut history = collections::VecDeque::new();
    history.push_front(lb.clone());

    clog.nice_to_know("Running");

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
