#[cfg(feature = "terminal-display")]
pub mod terminal;

#[cfg(feature = "terminal-display")]
pub use terminal::*;

pub trait BaseDisplay {
    fn display_width(&self) -> usize;

    fn display_height(&self) -> usize;

    fn display_dimensions(&self) -> (usize, usize) {
        (self.display_width(), self.display_height())
    }
}
