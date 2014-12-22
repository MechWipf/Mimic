
//
//  Emulator
//! The emulator program.
//


extern crate jni;

use self::jni::{JavaVM, Class};
use minion::Minion;

use std::os;


/// The emulator class binding the JavaVM and terminal display.
pub struct Emulator {
	_jvm: JavaVM,
	java_class: Class,
	minions: Vec<Minion>,
	last_id: i32,
}


impl Emulator {

	/// Create a new emulator.
	pub fn new() -> Emulator {
		let classpaths = [
			Path::new("/Users/benanderson/Desktop/mimic/java"),
			Path::new("/Users/benanderson/Desktop/mimic/java/computercraft.jar"),
		];

		let mut jvm = JavaVM::new(&classpaths).unwrap();
		jvm.set_calls_destructor(false);

		let class = jvm.class("Minion").unwrap();

		Emulator {
			_jvm: jvm,
			java_class: class,
			minions: Vec::new(),
			last_id: -1,
		}
	}

	/// Create a new minion with an automatically assigned ID.
	pub fn new_minion(&mut self) {
		self.last_id += 1;
		self.minions.push(Minion::new(
			self.last_id as u32,
			true,
			&self.java_class
		));
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
			for minion in self.minions.iter_mut() {
				minion.advance();
				minion.trigger_events();
			}
		}
	}

}
