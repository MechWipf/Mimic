
//
//  Mimic
//! A ComputerCraft emulator.
//

#![feature(macro_rules)]


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
	storage::create();
	let potential = Config::from_file(&storage::config());

	match potential {
		Ok(config) => {
			let mut emulator = Emulator::new(&config);
			emulator.new_minion(true, false);
			emulator.run();
		},
		Err(message) => {
			println!("Configuration loading failed:\n{}", message);

			let err_window = ErrorWindow::new(&[
				"Failed to load configuration.",
				"Check the command line for more information.",
			]);

			while err_window.term.is_running() {
				err_window.update();
			}
		}
	}
}
