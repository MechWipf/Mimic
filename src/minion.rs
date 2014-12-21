
//
//  Minion
//! A single emulated computer.
//


extern crate terminal;
extern crate jni;

use std::str::CharRange;

use terminal::Manager;
use jni::{Class, Object, Value, Type};
use color;


/// The default width in cells of a computer.
const DEFAULT_WIDTH: u32 = 51;

/// The default height in cells of a computer.
const DEFAULT_HEIGHT: u32 = 19;


/// A single emulated computer.
pub struct Minion {
	window_id: u32,
	java_object: Object,
}


impl Minion {

	/// Create a new minion.
	pub fn new(id: u32, is_color: bool, manager: &mut Manager, computer_class: &Class) -> Minion {
		let java_object = computer_class.instance(&[
			Value::Int(id as i32),
			Value::Boolean(is_color)
		]).unwrap();

		let title = format!("Computer {}", id);
		let window_id = manager.terminal(title.as_slice(), DEFAULT_WIDTH, DEFAULT_HEIGHT);

		Minion {
			window_id: window_id,
			java_object: java_object,
		}
	}

	/// Update the cell contents on a particular line with a Java text and color string.
	fn update_line(&self, manager: &mut Manager, line: u32, text: &str, color: &str) {
		let mut letter_index = 0;
		for x in range(0, DEFAULT_WIDTH) {
			let CharRange {ch, next} = text.char_range_at(letter_index);
			let foreground = color.char_at(x as uint);
			let foreground_color = color::to_hex(foreground);
			let background = color.char_at((x + DEFAULT_WIDTH) as uint);
			let background_color = color::to_hex(background);

			manager.set_character(self.window_id, ch, x, line);
			manager.set_background(self.window_id, foreground_color, x, line);
			manager.set_foreground(self.window_id, background_color, x, line);

			letter_index = next;
		}
	}

	/// Update the contents of the window's cells and advance the computer's tick count.
	pub fn advance(&self, manager: &mut Manager) {
		self.java_object.call("advance", &[], Type::Void).unwrap();

		let text = self.java_object.call("getLine", &[Value::Int(0)], Type::String)
			.unwrap();
		let color = self.java_object.call("getColorLine", &[Value::Int(1)], Type::String)
			.unwrap();

		self.update_line(manager, 0, text.to_string().as_slice(), color.to_string().as_slice());
	}

}


impl Drop for Minion {

	fn drop(&mut self) {
		self.java_object.call("destroy", &[], Type::Void).unwrap();
	}

}
