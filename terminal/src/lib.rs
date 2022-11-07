//! # terminal
//!
//! A library for interacting with terminal.

use libc::{winsize, termios, ICANON, TIOCGWINSZ,
	ECHO, STDOUT_FILENO, TCSANOW, F_SETFL, O_NONBLOCK};
use std::process;
use ctrlc;

/// Clear the all terminal
#[inline]
pub fn clear_screen() {
	println!("\x1b[2J");
}

/// Reset styles and colors
#[inline]
pub fn reset_mods() {
	print!("\x1b[0m");
}

/// Set foreground color as RGB
#[inline]
pub fn fcolor_rgb(r: u8, g: u8, b: u8) {
	print!("\x1b[38;2;{r};{g};{b}m");
}

/// Set background color as RGB
#[inline]
pub fn bcolor_rgb(r: u8, g: u8, b: u8) {
	print!("\x1b[48;2;{r};{g};{b}m");
}

/// Move cursor to position (x,y)
#[inline]
pub fn cursor_move(x: usize, y: usize) {
	print!("\x1b[{y};{x}H");
}

/// Make cursor {in}visible
pub fn cursor_visible(enable: bool) {
	if enable { 
		println!("\x1b[?25h");
	} else {
		println!("\x1b[?25l");
	}
}

/// Restore 
pub fn catch_ctrlc() -> Result<(),()> {
	match ctrlc::set_handler(|| {
		canon_and_echo(true);
		cursor_visible(true);
		nonblocking_stdin(false);
		clear_screen();
		process::exit(0);
	}) {
		Ok(_) => Ok(()),
		Err(_) => Err(()),
	}
}

/// 
pub fn nonblocking_stdin(enable: bool) -> Result<(),()> {
	let ret: i32;
	if enable {
		ret = unsafe {
			libc::fcntl(0, F_SETFL, O_NONBLOCK)
		}
	} 
	else {
		ret = unsafe {
			libc::fcntl(0, F_SETFL, O_NONBLOCK)
		}
	}
	if ret == -1 {

		Err(())
	}
	else {
		Ok(())
	}
}

/// {Dis/En}able canonical and echo
pub fn canon_and_echo(enable: bool) -> Result<(),()> {

	let mut config = termios {
	    c_iflag: 0,
	    c_oflag: 0,
	    c_cflag: 0,
	    c_lflag: 0,
	    c_line: 0,
	    c_cc: [0; 32],
	    c_ispeed: 0,
	    c_ospeed: 0,
	};

	if unsafe {
		libc::tcgetattr(STDOUT_FILENO, &mut config)
	} == -1 {
		return Err(())
	}
	if enable {
		config.c_lflag |= ICANON | ECHO;
	} else {
		config.c_lflag &= !(ICANON | ECHO);
	}
	if unsafe {
		libc::tcsetattr(STDOUT_FILENO, TCSANOW, &mut config)
	} == -1 {
		Err(())
	} else {
		Ok(())
	}
}

/// Get width and height of the terminal
pub fn get_terminal_size() -> Result<(usize, usize), ()> {
	let mut config = winsize  {
    	ws_row: 0,
    	ws_col: 0,
    	ws_xpixel: 0,
    	ws_ypixel: 0,
	};
	if unsafe {
		libc::ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut config)
	} == -1 {
		return Err(());
	}
	Ok((config.ws_col.into(), config.ws_row.into()))
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn add_one() {
        assert_eq!(1, 1);
    }
}
