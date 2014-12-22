
//
//  Mimic
//! A ComputerCraft emulator.
//


use emulator::Emulator;

mod minion;
mod color;
mod emulator;
mod convert;
mod storage;
mod config;


fn main() {
	storage::create();

	let mut emulator = Emulator::new();
	emulator.new_minion();
	emulator.run();
}
