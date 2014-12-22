
//
//  Emulator
//! The emulator program.
//


extern crate jni;

use self::jni::{JavaVM, Class};
use config::Config;
use minion::{Minion, Action, Options};

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

		// Get the minion's width and height
		let (width, height) = if pocket {
			(self.config.pocket_width, self.config.pocket_height)
		} else {
			(self.config.computer_width, self.config.computer_height)
		};

		// Create its title
		let title = if pocket {
			format!("Pocket Computer {}", self.last_id)
		} else {
			format!("Computer {}", self.last_id)
		};

		// Create its options
		let options = Options {
			id: self.last_id as u32,
			advanced: advanced,
			title: title,
			width: width,
			height: height,
			space_limit: self.config.space_limit,
		};

		// Create the minion itself
		let minion = if self.minions.len() == 0 {
			Minion::new(&options, &self.java_class)
		} else {
			Minion::from_parent(&self.minions[0], &options, &self.java_class)
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

			// Advance each minion
			for minion in self.minions.iter_mut() {
				minion.advance();
				let potential = minion.trigger_events();
				match potential {
					Some(action) => actions.push(action),
					None => {},
				}
			}

			// Handle any returned actions
			for action in actions.iter() {
				match action {
					&Action::NewComputer(advanced) =>
						self.new_minion(advanced, false),
					&Action::NewPocketComputer(advanced) =>
						self.new_minion(advanced, true),
				}
			}
		}
	}

}
