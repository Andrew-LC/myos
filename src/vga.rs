#[allow(unused_imports)]
use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use core::cell::UnsafeCell;


#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let c = c as u8;
    for i in 0..n {
        *s.add(i) = c;
    }
    s
}


/* Hardware text mode color constants. */
#[allow(dead_code)]
enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}

fn vga_entry_color(fg: VgaColor, bg: VgaColor) -> u8 {
    fg as u8 | (bg as u8) << 4
}


fn vga_entry(uc: u8, color: u8) -> u16 {
    uc as u16 | (color as u16) << 8
}

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;


pub struct TerminalWriter {
    terminal_row: usize,
    terminal_column: usize,
    terminal_color: u8,
    terminal_buffer: UnsafeCell<*mut u16>,
}

unsafe impl Send for TerminalWriter {}
unsafe impl Sync for TerminalWriter {}

impl TerminalWriter {
    pub fn new() -> TerminalWriter {
        let terminal_color = vga_entry_color(VgaColor::LightGreen, VgaColor::Black);
        let terminal_buffer = UnsafeCell::new(0xB8000 as *mut u16);

        let writer = TerminalWriter {
            terminal_row: 0,
            terminal_column: 0,
            terminal_color,
            terminal_buffer,
        };

        // Clear screen
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let index = y * VGA_WIDTH + x;
                unsafe {
                    *(*writer.terminal_buffer.get()).add(index) = vga_entry(b' ', terminal_color);
                }
            }
        }

        writer
    }

    #[allow(dead_code)]
    fn set_color(&mut self, color: u8) {
        self.terminal_color = color;
    }

    fn putentryat(&mut self, c: u8, color: u8, x: usize, y: usize) {
        let index = y * VGA_WIDTH + x;
        unsafe {
            *(*self.terminal_buffer.get()).add(index) = vga_entry(c, color);
        }
    }

    fn putchar(&mut self, c: u8) {
	match c {
	    b'\n' => {
		self.terminal_column = 0;
		self.terminal_row += 1;
		if self.terminal_row == VGA_HEIGHT {
		    self.terminal_row = 0; // or scroll if implemented
		}
	    }
	    _ => {
		self.putentryat(
		    c,
		    self.terminal_color,
		    self.terminal_column,
		    self.terminal_row,
		);
		self.terminal_column += 1;
		if self.terminal_column == VGA_WIDTH {
		    self.terminal_column = 0;
		    self.terminal_row += 1;
		    if self.terminal_row == VGA_HEIGHT {
			self.terminal_row = 0;
		    }
		}
	    }
	}
    }


    pub fn write(&mut self, data: &str) {
        for c in data.bytes() {
            self.putchar(c);
        }
    }
}

impl Write for TerminalWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
	self.write(s);
	Ok(())
    }
}

lazy_static! {
    pub static ref TERMINAL_WRITER: Mutex<TerminalWriter> =
        Mutex::new(TerminalWriter::new());
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use spin::MutexGuard;
    let mut writer: MutexGuard<TerminalWriter> = TERMINAL_WRITER.lock();
    writer.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::vga::_print(core::format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($fmt:expr) => {
        $crate::print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::print!(concat!($fmt, "\n"), $($arg)*)
    };
}
