
//
//  Mimic
//! A ComputerCraft emulator.
//


extern crate terminal;
extern crate jni;
extern crate serialize;

use emulator::Emulator;
use config::Config;
use error::ErrorWindow;

mod minion;
mod color;
mod emulator;
mod convert;
mod storage;
mod config;
mod error;


fn main() {
	// Create the storage directory and default configuration file if needed.
	storage::create();

	// Load the configuration.
	let potential = Config::from_file(&storage::config());

	match potential {
		Ok(config) => {
			// Successfully loaded. Start the emulator.
			let mut emulator = Emulator::new(&config);
			emulator.new_minion(true, false);
			emulator.run();
		},
		Err(message) => {
			// Failed.
			println!("Configuration loading failed:\n{}", message);

			let mut err_window = ErrorWindow::new(&[
				"Failed to load configuration.",
				"Check the command line for more information.",
			]);

			while err_window.term.is_running() {
				err_window.update();
			}
		}
	}
}
