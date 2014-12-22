
//
//  Color
//! Color values and conversion.
//


const WHITE: u32 = 0xf0f0f0;
const ORANGE: u32 = 0xf2b233;
const MAGENTA: u32 = 0xe57fd8;
const LIGHT_BLUE: u32 = 0x99b2f2;
const YELLOW: u32 = 0xdede6c;
const LIME: u32 = 0x7fcc19;
const PINK: u32 = 0xf2b2cc;
const GRAY: u32 = 0x4c4c4c;
const LIGHT_GRAY: u32 = 0x999999;
const CYAN: u32 = 0x4c99b2;
const PURPLE: u32 = 0xb266e5;
const BLUE: u32 = 0x3366cc;
const BROWN: u32 = 0x7f664c;
const GREEN: u32 = 0x57a64e;
const RED: u32 = 0xcc4c4c;
const BLACK: u32 = 0x000000;


/// Converts a hex color character into a hex color code.
pub fn character_to_hex(character: char) -> u32 {
	number_to_hex(character.to_digit(16).unwrap() as i32)
}

/// Converts a number into a hex color code.
pub fn number_to_hex(number: i32) -> u32 {
	match number {
		0 => BLACK,
		1 => RED,
		2 => GREEN,
		3 => BROWN,
		4 => BLUE,
		5 => PURPLE,
		6 => CYAN,
		7 => LIGHT_GRAY,
		8 => GRAY,
		9 => PINK,
		10 => LIME,
		11 => YELLOW,
		12 => LIGHT_BLUE,
		13 => MAGENTA,
		14 => ORANGE,
		15 => WHITE,
		_ => panic!("Unrecognized color {}", number),
	}
}
