
//
//  Minion
//! A single emulated computer.
//


extern crate terminal;
extern crate jni;

use std::os;
use std::str::CharRange;

use self::terminal::Terminal;
use self::terminal::event::{Event, Key, MouseButton};
use self::terminal::window::time;
use self::jni::{Class, Object, Value, Type};

use convert;
use color;
use storage;


/// The duration in seconds between each cursor flash.
const CURSOR_FLASH_RATE: f64 = 0.5;

/// The duration to hold a timed keyboard shortcut for.
const TIMED_SHORTCUT_DURATION: f64 = 1.0;

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
	width: u32,
	height: u32,

	shortcut_timer: f64,
	shortcut_key: Key,
}


impl Minion {

	/// Create a new minion.
	pub fn new(id: u32, is_color: bool, width: u32, height: u32, computer_class: &Class)
			-> Minion {
		let storage_dir = storage::storage().as_str().unwrap().to_string();
		let java_object = computer_class.instance(&[
			Value::Int(id as i32),
			Value::Boolean(is_color),
			Value::Int(width as i32),
			Value::Int(height as i32),
			Value::String(storage_dir),
		]).unwrap();

		let title = format!("Computer {}", id);
		let term = Terminal::new(
			title.as_slice(),
			width,
			height
		);

		Minion {
			term: term,
			java_object: java_object,

			cursor_flash: true,
			cursor_flash_swap_time: time(),
			width: width,
			height: height,

			shortcut_timer: -1.0,
			shortcut_key: Key::A,
		}
	}


	//
	//  Update
	//

	/// Update the cell contents on a particular line with a Java text and color string.
	fn update_line(&mut self, line: u32, text: &str, color: &str) {
		let mut letter_index = 0;
		for x in range(0, self.width) {
			let CharRange {ch, next} = text.char_range_at(letter_index);
			let foreground = color.char_at((x + self.width) as uint);
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

	/// Updates any timed keyboard shortcuts.
	fn update_timed_shortcuts(&mut self) {
		let command_down =
			self.term.window.is_key_down(Key::LeftControl) ||
			self.term.window.is_key_down(Key::RightControl);

		if self.shortcut_timer > -1.0 {
			if self.term.window.is_key_down(self.shortcut_key) && command_down {
				let current_time = time();
				if current_time - self.shortcut_timer >= TIMED_SHORTCUT_DURATION {
					match self.shortcut_key {
						Key::R => self.reboot(),
						Key::S => self.shutdown(),
						Key::T => self.terminate(),
						_ => {},
					}

					self.shortcut_timer = -1.0;
				}
			} else {
				self.shortcut_timer = -1.0;
			}
		} else if command_down {
			if self.term.window.is_key_down(Key::R) {
				self.shortcut_timer = time();
				self.shortcut_key = Key::R;
			} else if self.term.window.is_key_down(Key::S) {
				self.shortcut_timer = time();
				self.shortcut_key = Key::S;
			} else if self.term.window.is_key_down(Key::T) {
				self.shortcut_timer = time();
				self.shortcut_key = Key::T;
			}
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

		for y in range(0, self.height) {
			let text = self.java_object.call("getLine", &[
				Value::Int(y as i32),
			], Type::String).unwrap().to_string();

			let color = self.java_object.call("getColorLine", &[
				Value::Int(y as i32),
			], Type::String).unwrap().to_string();

			self.update_line(y, text.as_slice(), color.as_slice());
		}

		self.update_cursor();
		self.update_timed_shortcuts();
	}


	//
	//  Events
	//

	/// Trigger any user events.
	pub fn trigger_events(&mut self) {
		for event in self.term.events().iter() {
			match event {
				&Event::KeyDown(key) => {
					self.trigger_key(key);
					self.trigger_shortcuts(key);
				},
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

	/// Handle keyboard shortcuts.
	fn trigger_shortcuts(&self, key: Key) {

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


	//
	//  Functions
	//

	/// Terminate the current program on the computer.
	pub fn terminate(&self) {
		self.java_object.call("terminate", &[], Type::Void).unwrap();
	}

	/// Shutdown the computer.
	pub fn shutdown(&self) {
		self.java_object.call("shutdown", &[], Type::Void).unwrap();
	}

	/// Reboot the computer.
	pub fn reboot(&self) {
		self.java_object.call("reboot", &[], Type::Void).unwrap();
	}

}
