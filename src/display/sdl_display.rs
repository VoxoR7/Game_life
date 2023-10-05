use super::displayable;
use crate::board;

extern crate sdl2;

const DEFAULT_WIDTH: usize = 1920;
const DEFAULT_HEIGHT: usize = 1080;

const DEFAULT_CONTROL_HEIGHT: usize = 120;

const DEFAULT_STOP_WIDTH: f64 = 25.0;
const DEFAULT_CONTINUE_WIDTH: f64 = 25.0;
const DEFAULT_STEP_WIDTH: f64 = 25.0;
const DEFAULT_QUIT_WIDTH: f64 = 25.0;

pub struct SdlDisplay {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event: sdl2::EventPump,
    touch_pressed: bool,
}

impl displayable::Displayable for SdlDisplay {
    fn print(&mut self, board: &board::Board) {
        let (row, col) = board.get_size();

        let cell_width = DEFAULT_WIDTH / col;
        let cell_height = (DEFAULT_HEIGHT - DEFAULT_CONTROL_HEIGHT) / row;

        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();

        // stop button
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        self.canvas
            .fill_rect(sdl2::rect::Rect::new(0, 0, (DEFAULT_WIDTH as f64 * (DEFAULT_STOP_WIDTH / 100.0)) as u32, DEFAULT_CONTROL_HEIGHT as u32))
            .expect("Couldn't draw to the screen !");

        // continue button
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
        self.canvas
            .fill_rect(sdl2::rect::Rect::new((DEFAULT_WIDTH as f64 * (DEFAULT_STOP_WIDTH / 100.0)) as i32, 0, (DEFAULT_WIDTH as f64 * (DEFAULT_CONTINUE_WIDTH / 100.0)) as u32, DEFAULT_CONTROL_HEIGHT as u32))
            .expect("Couldn't draw to the screen !");

        // step button
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 255));
        self.canvas
            .fill_rect(sdl2::rect::Rect::new((DEFAULT_WIDTH as f64 * (DEFAULT_STOP_WIDTH / 100.0) + DEFAULT_WIDTH as f64 * (DEFAULT_CONTINUE_WIDTH / 100.0)) as i32, 0, (DEFAULT_WIDTH as f64 * (DEFAULT_STEP_WIDTH / 100.0)) as u32, DEFAULT_CONTROL_HEIGHT as u32))
            .expect("Couldn't draw to the screen !");

        // quit button
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(127, 127, 127));
        self.canvas
            .fill_rect(sdl2::rect::Rect::new((DEFAULT_WIDTH as f64 * (DEFAULT_STOP_WIDTH / 100.0) + DEFAULT_WIDTH as f64 * (DEFAULT_CONTINUE_WIDTH / 100.0) + DEFAULT_WIDTH as f64 * (DEFAULT_STEP_WIDTH / 100.0)) as i32, 0, (DEFAULT_WIDTH as f64 * (DEFAULT_QUIT_WIDTH / 100.0)) as u32, DEFAULT_CONTROL_HEIGHT as u32))
            .expect("Couldn't draw to the screen !");

        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        

        for r in 0..row {
            for c in 0..col {
                match board.get_cell(r, c).unwrap() {
                    board::CellState::Dead => continue,
                    board::CellState::Alive => self.canvas
                        .fill_rect(sdl2::rect::Rect::new((cell_width * c) as i32, ((cell_height * r) + DEFAULT_CONTROL_HEIGHT) as i32, cell_width as u32, cell_height as u32))
                        .expect("Couldn't draw to the screen !"),
                }
            }
        }

        self.canvas.present();
    }

    fn control(&mut self) -> Option<crate::display::DisplayControl> {
        self.event.pump_events();

        if !self.event.mouse_state().left() && self.touch_pressed == true {
            self.touch_pressed = false;
        }

        if self.event.mouse_state().left() && self.touch_pressed == false {
            if self.event.mouse_state().y() < DEFAULT_CONTROL_HEIGHT as i32 {
                self.touch_pressed = true;
                let x = self.event.mouse_state().x();
                if x < (DEFAULT_WIDTH as f64 * (DEFAULT_STOP_WIDTH / 100.0)) as i32 {
                    return Some(crate::display::DisplayControl::STOP)
                }

                if x < (DEFAULT_WIDTH as f64 * (DEFAULT_STOP_WIDTH / 100.0) + DEFAULT_WIDTH as f64 * (DEFAULT_CONTINUE_WIDTH / 100.0)) as i32 {
                    return Some(crate::display::DisplayControl::CONTINUE)
                }

                if x < (DEFAULT_WIDTH as f64 * (DEFAULT_STOP_WIDTH / 100.0) + DEFAULT_WIDTH as f64 * (DEFAULT_CONTINUE_WIDTH / 100.0) + DEFAULT_WIDTH as f64 * (DEFAULT_STEP_WIDTH / 100.0)) as i32 {
                    return Some(crate::display::DisplayControl::STEP)
                }

                if x < (DEFAULT_WIDTH as f64 * (DEFAULT_STOP_WIDTH / 100.0) + DEFAULT_WIDTH as f64 * (DEFAULT_CONTINUE_WIDTH / 100.0) + DEFAULT_WIDTH as f64 * (DEFAULT_STEP_WIDTH / 100.0) + DEFAULT_WIDTH as f64 * (DEFAULT_QUIT_WIDTH / 100.0)) as i32 {
                    return Some(crate::display::DisplayControl::QUIT)
                }
            }
        }

        None
    }

    fn cell_control(&mut self, board: &board::Board) -> Option<(u32, u32)> {
        let (row, col) = board.get_size();
        self.event.pump_events();

        if !self.event.mouse_state().left() && self.touch_pressed == true {
            self.touch_pressed = false;
        }

        if self.event.mouse_state().left() && self.touch_pressed == false {
            if self.event.mouse_state().y() >= DEFAULT_CONTROL_HEIGHT as i32 {
                let y = self.event.mouse_state().y() - DEFAULT_CONTROL_HEIGHT as i32;
                let y_cell = y as u32 / ((DEFAULT_HEIGHT as u32 - DEFAULT_CONTROL_HEIGHT as u32) / row as u32);

                let x = self.event.mouse_state().x() as i32;
                let x_cell = x as u32 / (DEFAULT_WIDTH as u32 / col as u32);

                return Some((y_cell, x_cell));
            }
        }

        None
    }
}

impl SdlDisplay {
    pub fn new() -> SdlDisplay {
        let sdl_context = sdl2::init().expect("Couldn't create the sdl context !");
        let video_subsystem = sdl_context
            .video()
            .expect("Couldn't create de video subsystem !");

        let window = video_subsystem
            .window("Life Game by VoxoR", DEFAULT_WIDTH as u32, DEFAULT_HEIGHT as u32)
            .position_centered()
            .build()
            .expect("Couldn't create the window !");

        let canvas = window.
            into_canvas()
            .build()
            .expect("Couldn't create canvas !");

        let event_pump = sdl_context.
            event_pump().
            expect("Couldn't create the event pump !");

        SdlDisplay {
            canvas: canvas,
            event: event_pump,
            touch_pressed: false,
        }
    }
}
