use crate::BaseDisplay;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum TerminalColour {
    Black,
    DarkGray,
    Blue,
    LightBlue,
    Green,
    LightGreen,
    Cyan,
    LightCyan,
    Red,
    LightRed,
    Magenta,
    Pink,
    Brown,
    Yellow,
    LightGray,
    White,
}

pub trait TerminalDisplay: BaseDisplay {
    fn foreground_colour(&self) -> TerminalColour;

    fn background_colour(&self) -> TerminalColour;

    fn set_foreground_colour(&mut self, colour: TerminalColour);

    fn set_background_colour(&mut self, colour: TerminalColour);

    fn cursor(&self) -> (usize, usize);
    
    fn set_cursor(&self, cursor: (usize, usize));
    
    fn cursor_enabled(&self) -> bool;
    
    fn set_cursor_enabled(&self, enabled: bool);
    
    fn write_str_at(&self, s: &str, cursor: (usize, usize)) -> (usize, usize);

    fn write_str(&self, s: &str);
}
