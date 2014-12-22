
//
//  Mimic
//! A ComputerCraft emulator.
//

#![feature(macro_rules)]


use emulator::Emulator;
use config::Config;

mod minion;
mod color;
mod emulator;
mod convert;
mod storage;
mod config;


fn main() {
	storage::create();
	let config = Config::from_file(&storage::config());

	let mut emulator = Emulator::new(&config);
	emulator.new_minion();
	emulator.run();
}
