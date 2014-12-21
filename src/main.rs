
//
//  Mimic
//! A ComputerCraft emulator.
//


//
//  Compile Java code with: javac -cp java/computercraft.jar -d ./java src/java/Minion.java
//


extern crate terminal;
extern crate jni;

use terminal::Manager;
use jni::JavaVM;

use minion::Minion;

mod minion;
mod color;


fn main() {
	let mut jvm = JavaVM::new(&[
		Path::new("/Users/benanderson/Desktop/mimic/java"),
		Path::new("/Users/benanderson/Desktop/mimic/java/computercraft.jar"),
	]).unwrap();
	jvm.set_calls_destructor(false);

	let class = jvm.class("Minion").unwrap();
	let mut manager = Manager::new();
	let minion = Minion::new(0, true, &mut manager, &class);

	while manager.running() {
		minion.advance(&mut manager);
		manager.update(&mut []);
	}
}
