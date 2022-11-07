use std::time::Duration;
use rand::Rng;
use std::{io, thread};
use std::io::Read;
use std::collections::VecDeque;
use terminal;

const PARTITION_HEIGHT: usize = 15;
const PARTITION_ROW: &str = "||||| |||||";
const PARTITION_WIDTH: usize = PARTITION_ROW.len();

struct Pos {
	x: usize,
	y: usize,
}

struct Tap {
	p: Pos,
	c: char,
}

impl Tap {
	pub fn update(self: &mut Self, start_y: usize) -> Result<(), ()> {
		if self.p.y < start_y + PARTITION_HEIGHT - 1
		{
			self.p.y += 1;
			Ok(())
		}
		else { Err(()) }
	}
	pub fn draw(self: &Self, input: u8) {
		terminal::cursor_move(self.p.x, self.p.y);
		if input == self.c as u8 {
			terminal::bcolor_rgb(255,0,0);
			println!("{}", self.c);
			terminal::reset_mods();
		}
		else {
			println!("{}", self.c);
		}
		
	}
}

fn top_left_partition() -> (usize, usize) {

	let (width, height) = terminal::get_terminal_size().unwrap();

	assert!(width > PARTITION_WIDTH);
	assert!(height > PARTITION_HEIGHT);

	let (top_left_x, top_left_y) = (
		width / 2 - PARTITION_WIDTH / 2,
		height / 2 - PARTITION_HEIGHT / 2,
	);

	(top_left_x, top_left_y)
}

fn build_partition(top_left_x: usize, top_left_y: usize) -> String {

	let mut partition = String::new();

	for i in 0..PARTITION_HEIGHT {
		let cursor_move = format!(
			"\x1b[{};{}H",
			top_left_y + i, top_left_x
		);
		partition.push_str(&cursor_move);
		partition.push_str(PARTITION_ROW);
	}

	partition
}

fn which_finger(c: char) -> usize {
	match c {
		// auriculaire gauche
		'a' | 'A' | 'q' | 'Q' | 'z' | 'Z' => 0,
		// annulaire gauche
		'w' | 'W' | 's' | 'S' | 'x' | 'X' => 1,
		// majeur gauche
		'e' | 'E' | 'd' | 'D' | 'c' | 'C' => 2,
		// index gauche
		'r' | 'R' | 'f' | 'F' | 'v' | 'V' |
		't' | 'T' | 'g' | 'G' | 'b' | 'B' => 3,
		// pouce gauche
		' ' => 4,
		// pouce droit
		// index droit
		'y' | 'Y' | 'h' | 'H' | 'n' | 'N' |
		'u' | 'U' | 'j' | 'J' | 'm' | 'M' => 7,
		// majeur droit
		'i' | 'I' | 'k' | 'K' | ',' | '<' => 8,
		// annulaire droit
		'o' | 'O' | 'l' | 'L' | '.' | '>' => 9,
		// auriculaire droit
		'p' | 'P' | ';' | ':' | '/' | '?' => 10,
		_ => 6,
	}
}

fn main() {
	
	let mut need_pop_back = false;
	let mut script: VecDeque<Tap> = VecDeque::with_capacity(10);
	let (top_left_x, top_left_y) = top_left_partition();
	let partition = build_partition(top_left_x, top_left_y);

	terminal::catch_ctrlc();
	terminal::nonblocking_stdin(true);
	terminal::canon_and_echo(false);
	terminal::cursor_visible(false);
	loop {
		terminal::clear_screen();
		let mut input: [u8; 1] = [0];
		io::stdin().read(&mut input);
		println!("{}", partition);	
		for tap in &mut script {
			tap.draw(input[0]);
			match tap.update(top_left_y) {
				Ok(_) => continue,
				Err(_) => need_pop_back = true,
			};
		}
		if need_pop_back {
			script.pop_front();
		}
		let new_c = rand::thread_rng().gen_range('a'..='z');
		script.push_back( Tap {
			p: Pos {
				x: top_left_x + which_finger(new_c),
				y: top_left_y,
			},
			c: new_c,
		});
		thread::sleep(Duration::from_millis(500));
	}
}
