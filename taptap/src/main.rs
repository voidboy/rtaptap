use terminal;
use std::time::Duration;
use std::thread;

fn main() {
	terminal::clear_screen();
	terminal::fcolor_rgb(0,0,255);
	terminal::bcolor_rgb(255,0,0);
	terminal::cursor_move(10,20);
	terminal::canon_and_echo(false);
	terminal::cursor_visible(false);
	println!("Aurélien !");
	thread::sleep(Duration::from_secs(3));
	terminal::cursor_visible(true);
	terminal::canon_and_echo(true);
	terminal::reset_mods();
}
