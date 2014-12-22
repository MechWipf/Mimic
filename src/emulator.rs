
//
//  Emulator
//! The emulator program.
//


extern crate jni;

use self::jni::{JavaVM, Class};
use config::Config;
use minion::{Minion, Action};

use storage;


/// The emulator class binding the JavaVM and terminal display.
pub struct Emulator {
	_jvm: JavaVM,
	java_class: Class,
	minions: Vec<Minion>,
	last_id: i32,
	config: Config,
}


impl Emulator {

	/// Create a new emulator.
	pub fn new(config: &Config) -> Emulator {
		let mut jvm = JavaVM::new(storage::classpath().as_slice()).unwrap();
		jvm.set_calls_destructor(false);

		let class = jvm.class("Minion").unwrap();

		Emulator {
			_jvm: jvm,
			java_class: class,
			minions: Vec::new(),
			last_id: -1,
			config: config.clone(),
		}
	}

	/// Create a new minion with an automatically assigned ID.
	pub fn new_minion(&mut self, advanced: bool, pocket: bool) {
		self.last_id += 1;

		let (width, height) = if pocket {
			(self.config.pocket_width, self.config.pocket_height)
		} else {
			(self.config.computer_width, self.config.computer_height)
		};

		let title = if pocket {
			format!("Pocket Computer {}", self.last_id)
		} else {
			format!("Computer {}", self.last_id)
		};

		let minion = if self.minions.len() == 0 {
			Minion::new(
				self.last_id as u32,
				advanced,
				title.as_slice(),
				width,
				height,
				&self.java_class
			)
		} else {
			Minion::from_parent(
				&self.minions[0],
				self.last_id as u32,
				advanced,
				title.as_slice(),
				width,
				height,
				&self.java_class
			)
		};

		self.minions.push(minion);
	}

	/// Returns true if any minion is still running.
	pub fn is_running(&self) -> bool {
		let mut result = false;
		for minion in self.minions.iter() {
			result = result || minion.term.is_running();
		}
		result
	}

	/// Run the program, displaying the terminal windows and handling events.
	pub fn run(&mut self) {
		while self.is_running() {
			let mut actions = Vec::new();
			for minion in self.minions.iter_mut() {
				minion.advance();
				let action = minion.trigger_events();

				match action {
					Action::NoAction => {},
					action => actions.push(action),
				}
			}

			for action in actions.iter() {
				match action {
					&Action::NewComputer(advanced) =>
						self.new_minion(advanced, false),
					&Action::NewPocketComputer(advanced) =>
						self.new_minion(advanced, true),
					_ => {},
				}
			}
		}
	}

}
