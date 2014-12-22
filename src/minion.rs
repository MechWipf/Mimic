
//
//  Minion
//! A single emulated computer.
//


extern crate terminal;
extern crate jni;

use std::str::CharRange;

use self::terminal::Terminal;
use self::terminal::event::{Event, Key, MouseButton};
use self::terminal::window::time;
use self::jni::{Class, Object, Value, Type};

use convert;
use color;


/// The default width in cells of a computer.
const DEFAULT_WIDTH: u32 = 51;

/// The default height in cells of a computer.
const DEFAULT_HEIGHT: u32 = 19;

/// The duration in seconds between each cursor flash.
const CURSOR_FLASH_RATE: f64 = 0.5;

/// The valid characters that can be typed.
const VALID_CHARACTERS: &'static str = concat!(
	" !\"#$%&\'()*+,-./",
	"0123456789:;<=>?",
	"@ABCDEFGHIJKLMNO",
	"PQRSTUVWXYZ[\\]^_",
	"\'abcdefghijklmno",
	"pqrstuvwxyz{|}~",
);


/// A single emulated computer.
pub struct Minion {
	pub term: Terminal,
	java_object: Object,

	cursor_flash: bool,
	cursor_flash_swap_time: f64,
}


impl Minion {

	/// Create a new minion.
	pub fn new(id: u32, is_color: bool, computer_class: &Class) -> Minion {
		let java_object = computer_class.instance(&[
			Value::Int(id as i32),
			Value::Boolean(is_color)
		]).unwrap();

		let title = format!("Computer {}", id);
		let term = Terminal::new(title.as_slice(), DEFAULT_WIDTH, DEFAULT_HEIGHT);

		Minion {
			term: term,
			java_object: java_object,

			cursor_flash: true,
			cursor_flash_swap_time: time(),
		}
	}


	//
	//  Update
	//

	/// Update the cell contents on a particular line with a Java text and color string.
	fn update_line(&mut self, line: u32, text: &str, color: &str) {
		let mut letter_index = 0;
		for x in range(0, DEFAULT_WIDTH) {
			let CharRange {ch, next} = text.char_range_at(letter_index);
			let foreground = color.char_at((x + DEFAULT_WIDTH) as uint);
			let foreground_color = color::character_to_hex(foreground);
			let background = color.char_at(x as uint);
			let background_color = color::character_to_hex(background);

			self.term.character(ch, x, line);
			self.term.background(foreground_color, x, line);
			self.term.foreground(background_color, x, line);

			letter_index = next;
		}
	}

	/// Update the cursor position and visibility.
	fn update_cursor(&mut self) {
		let x = self.java_object.call("getCursorX", &[], Type::Int)
			.unwrap().to_i32();
		let y = self.java_object.call("getCursorY", &[], Type::Int)
			.unwrap().to_i32();
		let color = self.java_object.call("getCursorColor", &[], Type::Int)
			.unwrap().to_i32();
		let blink = self.java_object.call("getCursorBlink", &[], Type::Boolean)
			.unwrap().to_bool();
		let visible = blink && self.cursor_flash;
		let hex_color = color::number_to_hex(color);

		self.term.cursor_visibility(visible);
		if visible {
			self.term.cursor_position(x as u32, y as u32);
			self.term.cursor_color(hex_color);
		}
	}

	/// Update the contents of the window's cells and advance the computer's tick count.
	pub fn advance(&mut self) {
		self.java_object.call("advance", &[], Type::Void).unwrap();

		let current_time = time();
		if current_time - self.cursor_flash_swap_time >= CURSOR_FLASH_RATE {
			self.cursor_flash = !self.cursor_flash;
			self.cursor_flash_swap_time = current_time;
		}

		for y in range(0, DEFAULT_HEIGHT) {
			let text = self.java_object.call("getLine", &[Value::Int(y as i32)], Type::String)
				.unwrap().to_string();
			let color = self.java_object.call("getColorLine", &[Value::Int(y as i32)],
				Type::String).unwrap().to_string();

			self.update_line(y, text.as_slice(), color.as_slice());
		}

		self.update_cursor();
	}


	//
	//  Events
	//

	/// Trigger any user events.
	pub fn trigger_events(&mut self) {
		for event in self.term.events().iter() {
			match event {
				&Event::KeyDown(key) =>
					self.trigger_key(key),
				&Event::Character(character) =>
					self.trigger_char(character),
				&Event::MouseDown(x, y, button) =>
					self.trigger_mouse("mouseClickEvent", x, y, button),
				&Event::MouseDrag(x, y, button) =>
					self.trigger_mouse("mouseDragEvent", x, y, button),
				&Event::MouseScroll(_, y_delta) =>
					self.trigger_scroll(y_delta),
				_ => {},
			}
		}
	}

	/// Trigger a key down event.
	pub fn trigger_key(&self, key: Key) {
		let potential = convert::key_to_lwjgl(key);
		if let Some(converted) = potential {
			self.java_object.call("keyEvent", &[
				Value::Int(converted),
			], Type::Void).unwrap();
		}
	}

	/// Trigger a char event.
	pub fn trigger_char(&self, character: char) {
		if let Some(_) = VALID_CHARACTERS.find(character) {
			self.java_object.call("charEvent", &[
				Value::String(character.to_string()),
			], Type::Void).unwrap();
		}
	}

	/// Trigger a mouse click or drag event.
	pub fn trigger_mouse(&self, name: &str, x: f32, y: f32, button: MouseButton) {
		let converted_button = convert::button_to_lwjgl(button);
		let (cell_x, cell_y) = self.term.to_cell_position(x, y);

		self.java_object.call(name, &[
			Value::Int(converted_button),
			Value::Int(cell_x as i32),
			Value::Int(cell_y as i32)
		], Type::Void).unwrap();
	}

	/// Trigger a mouse scroll event.
	pub fn trigger_scroll(&self, y_delta: f32) {
		let x = self.term.window.cursor_x();
		let y = self.term.window.cursor_y();
		let (cell_x, cell_y) = self.term.to_cell_position(x, y);
		let direction = if y_delta < 0.0 { -1 } else { 1 };

		self.java_object.call("mouseScrollEvent", &[
			Value::Int(direction),
			Value::Int(cell_x as i32),
			Value::Int(cell_y as i32),
		], Type::Void).unwrap();
	}

}
