mod displayable;
mod sdl_display;
mod term_display;
use self::term_display::TermDisplay;
use self::sdl_display::SdlDisplay;

pub enum DisplayType {TERM, SDL}

#[derive(PartialEq, Debug)]
pub enum DisplayControl {CONTINUE, STOP, STEP, QUIT}

pub fn new(display_type: DisplayType) -> Box<dyn displayable::Displayable> {
    match display_type {
        DisplayType::TERM => Box::new(TermDisplay::new()),
        DisplayType::SDL => Box::new(SdlDisplay::new()),
    }
}

impl std::fmt::Display for DisplayControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayControl::CONTINUE => write!(f, "CONTINUE"),
            DisplayControl::STOP => write!(f, "STOP"),
            DisplayControl::STEP => write!(f, "STEP"),
            DisplayControl::QUIT => write!(f, "QUIT"),
        }
    }
}
