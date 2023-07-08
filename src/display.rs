mod display;
mod sdl_display;
mod term_display;
use self::term_display::TermDisplay;

pub fn new() -> Box<dyn display::Displayable> {
    Box::new(TermDisplay::new())
}
