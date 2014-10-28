extern crate tcod;

use tcod::{Console, background_flag, key_code, Special, color};

fn main() {
	let mut con = Console::init_root(80, 50, "libtcod Rust tutorial", false);
	let mut exit = false;
	while !(Console::window_closed() || exit) {
		con.clear();
		con.put_char_ex(40, 25, '@', color::blue, color::darkest_green);
		Console::flush();
		let keypress = Console::wait_for_keypress(true);
		match keypress.key {
			Special(key_code::Escape) => exit = true,
			_ => {}
		}
	}
}
