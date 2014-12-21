
//
//  Color
//! Color values and conversion.
//


const WHITE: u32 = 0xF0F0F0;
const ORANGE: u32 = 0xF2B233;
const MAGENTA: u32 = 0xE57FD8;
const LIGHT_BLUE: u32 = 0x99B2F2;
const YELLOW: u32 = 0xDEDE6C;
const LIME: u32 = 0x7FCC19;
const PINK: u32 = 0xF2B2CC;
const GRAY: u32 = 0x4C4C4C;
const LIGHT_GRAY: u32 = 0x999999;
const CYAN: u32 = 0x4C99B2;
const PURPLE: u32 = 0xB266E5;
const BLUE: u32 = 0x3366CC;
const BROWN: u32 = 0x7F664C;
const GREEN: u32 = 0x57A64E;
const RED: u32 = 0xCC4C4C;
const BLACK: u32 = 0x000000;


/// Converts a hex color character into a hex color code.
pub fn to_hex(character: char) -> u32 {
	match character {
		'0' => BLACK,
		'1' => RED,
		'2' => GREEN,
		'3' => BROWN,
		'4' => BLUE,
		'5' => PURPLE,
		'6' => CYAN,
		'7' => LIGHT_GRAY,
		'8' => GRAY,
		'9' => PINK,
		'a' => LIME,
		'b' => YELLOW,
		'c' => LIGHT_BLUE,
		'd' => MAGENTA,
		'e' => ORANGE,
		'f' => WHITE,
		_ => panic!("Unrecognized color code {}", character),
	}
}
