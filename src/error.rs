
//
//  Error
//! Displays a window with an error message.
//


extern crate terminal;

use self::terminal::Terminal;


/// The width of the error window in cells.
const WIDTH: u32 = 51;

/// The height of the error window in cells.
const HEIGHT: u32 = 19;


/// The error window.
pub struct ErrorWindow {
	pub term: Terminal,
}


impl ErrorWindow {

	/// Creates a new error window with the given message.
	pub fn new(messages: &[&str]) -> ErrorWindow {
		let mut term = Terminal::new("Error", WIDTH, HEIGHT);
		term.cursor_visibility(false);
		term.write("Error", (WIDTH - 5) / 2, 3);

		let mut i = 0;
		for message in messages.iter() {
			term.write(*message, (WIDTH - message.len() as u32) / 2, 6 + i);
			i += 1;
		}

		ErrorWindow {
			term: term,
		}
	}

	/// Renders the message and polls events.
	pub fn update(&self) {
		self.term.events();
	}

}
