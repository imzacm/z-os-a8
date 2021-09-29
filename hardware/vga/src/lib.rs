#![no_std]
#![deny(unsafe_code)]
#![deny(clippy::all)]

use z_hardware_traits::{BaseDisplay, TerminalDisplay, TerminalColour};
use vga::colors::{Color16, TextModeColor};
use vga::writers::{ScreenCharacter, TextWriter, Text80x25, Screen};
use conquer_once::OnceCell;
use spin::Mutex;

static TERMINAL_INSTANCE: OnceCell<TerminalInstance> = OnceCell::uninit();

fn terminal_colour_to_vga(colour: TerminalColour) -> Color16 {
    match colour {
        TerminalColour::Black => Color16::Black,
        TerminalColour::DarkGray => Color16::DarkGrey,
        TerminalColour::Blue => Color16::Blue,
        TerminalColour::LightBlue => Color16::LightBlue,
        TerminalColour::Green => Color16::Green,
        TerminalColour::LightGreen => Color16::LightGreen,
        TerminalColour::Cyan => Color16::Cyan,
        TerminalColour::LightCyan => Color16::LightCyan,
        TerminalColour::Red => Color16::Red,
        TerminalColour::LightRed => Color16::LightRed,
        TerminalColour::Magenta => Color16::Magenta,
        TerminalColour::Pink => Color16::Pink,
        TerminalColour::Brown => Color16::Brown,
        TerminalColour::Yellow => Color16::Yellow,
        TerminalColour::LightGray => Color16::LightGrey,
        TerminalColour::White => Color16::White,
    }
}

struct TerminalInstance {
    text_mode: Text80x25,
    palette: Mutex<(TerminalColour, TerminalColour)>,
    cursor: Mutex<(usize, usize)>,
    cursor_enabled: Mutex<bool>,
}

impl TerminalInstance {
    fn init() -> &'static Self {
        let text_mode = Text80x25::default();
        text_mode.set_mode();
        text_mode.clear_screen();

        let palette = Mutex::new((TerminalColour::White, TerminalColour::Black));
        let cursor = Mutex::new((0, 0));
        let cursor_enabled = Mutex::new(true);
        TERMINAL_INSTANCE.get_or_init(|| Self { text_mode, palette, cursor, cursor_enabled })
    }
}

#[derive(Debug)]
pub struct VgaTerminalDisplay;

impl Default for VgaTerminalDisplay {
    fn default() -> Self {
        TerminalInstance::init();
        Self
    }
}

impl BaseDisplay for VgaTerminalDisplay {
    fn display_width(&self) -> usize {
        Text80x25::WIDTH
    }

    fn display_height(&self) -> usize {
        Text80x25::HEIGHT
    }
}

impl TerminalDisplay for VgaTerminalDisplay {
    fn foreground_colour(&self) -> TerminalColour {
        let lock = TERMINAL_INSTANCE.get().unwrap().palette.lock();
        lock.0
    }

    fn background_colour(&self) -> TerminalColour {
        let lock = TERMINAL_INSTANCE.get().unwrap().palette.lock();
        lock.1
    }

    fn set_foreground_colour(&mut self, colour: TerminalColour) {
        let mut lock = TERMINAL_INSTANCE.get().unwrap().palette.lock();
        (*lock).0 = colour;
    }

    fn set_background_colour(&mut self, colour: TerminalColour) {
        let mut lock = TERMINAL_INSTANCE.get().unwrap().palette.lock();
        (*lock).1 = colour;
    }

    fn cursor(&self) -> (usize, usize) {
        *TERMINAL_INSTANCE.get().unwrap().cursor.lock()
    }

    fn set_cursor(&self, cursor: (usize, usize)) {
        *TERMINAL_INSTANCE.get().unwrap().cursor.lock() = cursor;
    }

    fn cursor_enabled(&self) -> bool {
        *TERMINAL_INSTANCE.get().unwrap().cursor_enabled.lock()
    }

    fn set_cursor_enabled(&self, enabled: bool) {
        let instance = TERMINAL_INSTANCE.get().unwrap();
        let was_enabled = core::mem::replace(&mut *instance.cursor_enabled.lock(), enabled);
        if enabled == was_enabled {
            return;
        }
        if enabled { instance.text_mode.enable_cursor() } else { instance.text_mode.disable_cursor() };
    }

    fn write_str_at(&self, s: &str, mut cursor: (usize, usize)) -> (usize, usize) {
        let instance = TERMINAL_INSTANCE.get().unwrap();
        let colour = *instance.palette.lock();
        let colour = (terminal_colour_to_vga(colour.0), terminal_colour_to_vga(colour.1));
        let colour = TextModeColor::new(colour.0, colour.1);
        for byte in s.bytes() {
            let char = ScreenCharacter::new(byte, colour);
            instance.text_mode.write_character(cursor.0, cursor.1, char);

            if cursor.0 < Text80x25::WIDTH - 1 {
                cursor.0 += 1;
            } else if cursor.1 < Text80x25::HEIGHT - 1 {
                cursor.0 = 0;
                cursor.1 += 1;
            } else {
                cursor.0 = 0;
                let (_vga, frame_buffer) = instance.text_mode.get_frame_buffer();
                let char = ScreenCharacter::new(b' ', colour);

                const LAST_ROW_START: usize = Text80x25::SIZE - Text80x25::WIDTH;
                for i in LAST_ROW_START..Text80x25::SIZE {
                    #[allow(unsafe_code)] unsafe { frame_buffer.add(i).write_volatile(char) };
                }
            }
        }
        cursor
    }

    fn write_str(&self, s: &str) {
        let instance = TERMINAL_INSTANCE.get().unwrap();
        let cursor = *instance.cursor.lock();
        let cursor = self.write_str_at(s, (cursor.0, cursor.1));
        instance.text_mode.set_cursor_position(cursor.0, cursor.1);
    }
}
