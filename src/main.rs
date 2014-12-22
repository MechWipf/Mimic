
//
//  Mimic
//! A ComputerCraft emulator.
//


//
//  Compile Java code with: javac -cp java/computercraft.jar -d ./java src/java/Minion.java
//


use emulator::Emulator;

mod minion;
mod color;
mod emulator;
mod convert;


fn main() {
	let mut emulator = Emulator::new();
	emulator.new_minion();
	emulator.run();
}
