
//
//  Minion
//! A single emulated computer.
//


use std::os;
use std::str::CharRange;
use std::io::timer;
use std::time::duration::Duration;

use terminal::Terminal;
use terminal::event::{Event, Modifier, Key, MouseButton};
use terminal::window::time;
use jni::{Class, Object, Value, Type};

use convert;
use color;
use storage;


/// The duration in seconds between each cursor flash.
const CURSOR_FLASH_RATE: f64 = 0.5;

/// The duration to hold a timed keyboard shortcut for.
const TIMED_SHORTCUT_DURATION: f64 = 1.0;

/// The desired time for one frame.
const DESIRED_FRAME_DURATION: f64 = 1.0 / 60.0;

/// The valid characters that can be typed.
const VALID_CHARACTERS: &'static str = concat!(
	" !\"#$%&\'()*+,-./",
	"0123456789:;<=>?",
	"@ABCDEFGHIJKLMNO",
	"PQRSTUVWXYZ[\\]^_",
	"`abcdefghijklmno",
	"pqrstuvwxyz{|}~",
);


/// An action for the emulator to perform.
#[deriving(PartialEq)]
pub enum Action {
	/// Arguments: advanced
	NewComputer(bool),

	/// Arguments: advanced
	NewPocketComputer(bool),
}


/// Minion initialization options.
#[deriving(Clone, Show)]
pub struct Options {
	pub id: u32,
	pub advanced: bool,
	pub title: String,
	pub width: u32,
	pub height: u32,
	pub space_limit: u64,
}


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

	modem_attached: bool,
	previous_drag_x: i32,
	previous_drag_y: i32,
	advance_time: f64,
}


impl Minion {

	/// Create a new minion.
	pub fn new(options: &Options, computer_class: &Class) -> Minion {
		let term = Terminal::new(
			options.title.as_slice(),
			options.width,
			options.height
		);

		Minion::from_term(term, options, computer_class)
	}

	/// Create a new minion from a parent minion.
	pub fn from_parent(parent: &Minion, options: &Options, computer_class: &Class) -> Minion {
		let term = Terminal::from_parent(
			&parent.term,
			options.title.as_slice(),
			options.width,
			options.height
		);

		Minion::from_term(term, options, computer_class)
	}

	/// Create a minion from a terminal.
	fn from_term(term: Terminal, options: &Options, computer_class: &Class) -> Minion {
		let storage_dir = storage::storage().as_str().unwrap().to_string();
		let java_object = computer_class.instance(&[
			Value::Int(options.id as i32),
			Value::Boolean(options.advanced),
			Value::Int(options.width as i32),
			Value::Int(options.height as i32),
			Value::String(storage_dir),
			Value::Long(options.space_limit as i64),
		]).unwrap();

		let current_time = time();
		Minion {
			term: term,
			java_object: java_object,

			cursor_flash: true,
			cursor_flash_swap_time: current_time,
			width: options.width,
			height: options.height,

			shortcut_timer: -1.0,
			shortcut_key: Key::A,

			modem_attached: false,
			previous_drag_x: -1,
			previous_drag_y: -1,
			advance_time: current_time,
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
				if time() - self.shortcut_timer >= TIMED_SHORTCUT_DURATION {
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
		// Advance
		let current_time = time();
		let delta = current_time - self.advance_time;
		self.java_object.call("advance", &[Value::Double(delta)], Type::Void).unwrap();
		self.advance_time = current_time;

		// Check if the cursor flash needs inverting
		if current_time - self.cursor_flash_swap_time >= CURSOR_FLASH_RATE {
			self.cursor_flash = !self.cursor_flash;
			self.cursor_flash_swap_time = current_time;
		}

		// Render each terminal line
		for y in range(0, self.height) {
			// Get the text line and color line from Java
			let text = self.java_object.call("getLine", &[
				Value::Int(y as i32),
			], Type::String).unwrap().to_string();

			let color = self.java_object.call("getColorLine", &[
				Value::Int(y as i32),
			], Type::String).unwrap().to_string();

			self.update_line(y, text.as_slice(), color.as_slice());
		}

		// Update the cursor's position, visibility, and color
		self.update_cursor();

		// Update any timed shortcuts (reboot, shutdown, or terminate)
		self.update_timed_shortcuts();
	}


	//
	//  Events
	//

	/// Trigger any user events.
	pub fn trigger_events(&mut self) -> Option<Action> {
		let mut result = None;
		let mut suppress = false;
		let current_time = time();

		for event in self.term.events().iter() {
			match event {
				&Event::KeyDown(key, ref modifiers) => {
					let (r, s) = self.trigger_shortcuts(key, modifiers);
					result = r;
					suppress = s;
					if !suppress {
						self.trigger_key(key);
					}
				},
				&Event::Character(character) if !suppress =>
					self.trigger_char(character),
				&Event::MouseDown(x, y, button) =>
					self.trigger_mouse_click(x, y, button),
				&Event::MouseDrag(x, y, button) =>
					self.trigger_mouse_drag(x, y, button),
				&Event::MouseScroll(_, y_delta) =>
					self.trigger_scroll(y_delta),
				_ => {},
			}
		}

		// Sleep until the desired duration is up.
		let final_time = time();
		let delta = DESIRED_FRAME_DURATION - (final_time - current_time);
		if delta > 0.000001 {
			timer::sleep(Duration::nanoseconds((delta * 1000.0 * 1000.0 * 1000.0) as i64));
		}

		result
	}

	/// Handle keyboard shortcuts, returning an action for the emulator to act
	/// on, and whether to suppress the following key event.
	fn trigger_shortcuts(&mut self, key: Key, modifiers: &Vec<Modifier>)
			-> (Option<Action>, bool) {
		let mut command_down = false;
		let mut shift_down = false;

		// Convert the list of modifiers into boolean variables
		for modifier in modifiers.iter() {
			match *modifier {
				Modifier::Shift => shift_down = true,
				Modifier::Meta if os::consts::SYSNAME == "macos" => command_down = true,
				Modifier::Control => command_down = true,
				_ => {},
			}
		}

		if command_down {
			match key {
				// New normal computer
				Key::N if shift_down =>
					(Some(Action::NewComputer(false)), true),

				// New advanced computer
				Key::N =>
					(Some(Action::NewComputer(true)), true),

				// New normal pocket computer
				Key::B if shift_down =>
					(Some(Action::NewPocketComputer(false)), true),

				// New advanced pocket computer
				Key::B =>
					(Some(Action::NewPocketComputer(true)), true),

				// Attach a modem
				Key::A => {
					if self.modem_attached {
						self.detach_modem();
					} else {
						self.attach_modem();
					}

					(None, true)
				},

				// Paste
				Key::V => {
					self.paste();
					(None, true)
				},

				_ => (None, false)
			}
		} else {
			(None, false)
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
		// If we're allowed to type this character
		// Discard any non-ASCII characters. Too difficult.
		if let Some(_) = VALID_CHARACTERS.find(character) {
			self.java_object.call("charEvent", &[
				Value::String(character.to_string()),
			], Type::Void).unwrap();
		}
	}

	/// Trigger a mouse click event.
	pub fn trigger_mouse_click(&self, x: f32, y: f32, button: MouseButton) {
		let converted_button = convert::button_to_lwjgl(button);
		let (cell_x, cell_y) = self.term.to_cell_position(x, y);

		self.java_object.call("mouseClickEvent", &[
			Value::Int(converted_button),
			Value::Int(cell_x as i32 + 1),
			Value::Int(cell_y as i32 + 1),
		], Type::Void).unwrap();
	}

	/// Trigger a mouse drag event.
	pub fn trigger_mouse_drag(&mut self, x: f32, y: f32, button: MouseButton) {
		let converted_button = convert::button_to_lwjgl(button);
		let (cell_x, cell_y) = self.term.to_cell_position(x, y);
		let rx = cell_x as i32 + 1;
		let ry = cell_y as i32 + 1;

		if rx != self.previous_drag_x || ry != self.previous_drag_y {
			self.java_object.call("mouseDragEvent", &[
				Value::Int(converted_button),
				Value::Int(rx),
				Value::Int(ry),
			], Type::Void).unwrap();

			self.previous_drag_x = rx;
			self.previous_drag_y = ry;
		}
	}

	/// Trigger a mouse scroll event.
	pub fn trigger_scroll(&self, y_delta: f32) {
		let x = self.term.window.cursor_x();
		let y = self.term.window.cursor_y();
		let (cell_x, cell_y) = self.term.to_cell_position(x, y);
		let direction = if y_delta < 0.0 { 1 } else { -1 };

		self.java_object.call("mouseScrollEvent", &[
			Value::Int(direction),
			Value::Int(cell_x as i32),
			Value::Int(cell_y as i32),
		], Type::Void).unwrap();
	}


	//
	//  Functions
	//

	/// Attach a modem on the minion.
	pub fn attach_modem(&mut self) {
		self.java_object.call("attachModem", &[], Type::Void).unwrap();
		self.modem_attached = true;
	}

	/// Detach the modem from the minion.
	pub fn detach_modem(&mut self) {
		self.java_object.call("detachModem", &[], Type::Void).unwrap();
		self.modem_attached = false;
	}

	/// Paste the current clipboard contents string.
	pub fn paste(&self) {
		let contents = self.term.window.clipboard_contents();
		self.java_object.call("paste", &[Value::String(contents)], Type::Void).unwrap();
	}

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
