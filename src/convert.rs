
//
//  Convert
//! Convert to LWJGL key and mouse button values.
//


extern crate terminal;

use self::terminal::event::{Key, MouseButton};


/// Converts a key to an LWJGL key code.
pub fn key_to_lwjgl(key: Key) -> Option<i32> {
	match key {
		Key::Q => Some(16),
		Key::W => Some(17),
		Key::E => Some(18),
		Key::R => Some(19),
		Key::T => Some(20),
		Key::Y => Some(21),
		Key::U => Some(22),
		Key::I => Some(23),
		Key::O => Some(24),
		Key::P => Some(25),
		Key::A => Some(30),
		Key::S => Some(31),
		Key::D => Some(32),
		Key::F => Some(33),
		Key::G => Some(34),
		Key::H => Some(35),
		Key::J => Some(36),
		Key::K => Some(37),
		Key::L => Some(38),
		Key::Z => Some(44),
		Key::X => Some(45),
		Key::C => Some(46),
		Key::V => Some(47),
		Key::B => Some(48),
		Key::N => Some(49),
		Key::M => Some(50),

		Key::Number0 => Some(11),
		Key::Number1 => Some(2),
		Key::Number2 => Some(3),
		Key::Number3 => Some(4),
		Key::Number4 => Some(5),
		Key::Number5 => Some(6),
		Key::Number6 => Some(7),
		Key::Number7 => Some(8),
		Key::Number8 => Some(9),
		Key::Number9 => Some(10),

		Key::F1 => Some(59),
		Key::F2 => Some(60),
		Key::F3 => Some(61),
		Key::F4 => Some(62),
		Key::F5 => Some(63),
		Key::F6 => Some(64),
		Key::F7 => Some(65),
		Key::F8 => Some(66),
		Key::F9 => Some(67),
		Key::F10 => Some(68),
		Key::F11 => Some(87),
		Key::F12 => Some(88),
		Key::F13 => Some(100),
		Key::F14 => Some(101),
		Key::F15 => Some(102),

		Key::Space => Some(57),
		Key::Comma => Some(51),
		Key::Period => Some(52),
		Key::Backslash => Some(43),
		Key::Semicolon => Some(39),
		Key::Apostrophe => Some(40),
		Key::ForwardSlash => Some(53),
		Key::OpenBracket => Some(26),
		Key::CloseBracket => Some(27),
		Key::Equals => Some(13),
		Key::Minus => Some(12),
		Key::Tab => Some(15),
		Key::Backtick => Some(41),

		Key::Return => Some(28),
		Key::CapsLock => Some(58),
		Key::Escape => Some(1),
		Key::Backspace => Some(14),
		Key::Home => Some(199),
		Key::End => Some(207),
		Key::PageUp => Some(201),
		Key::PageDown => Some(209),

		Key::Up => Some(200),
		Key::Left => Some(203),
		Key::Right => Some(205),
		Key::Down => Some(208),

		Key::LeftShift => Some(42),
		Key::RightShift => Some(54),
		Key::LeftControl => Some(29),
		Key::RightControl => Some(157),
		Key::LeftAlt => Some(56),
		Key::RightAlt => Some(184),
		Key::LeftMeta => Some(219),
		Key::RightMeta => Some(220),
		_ => None,
	}
}

/// Converts a mouse button to an LWGJL one.
pub fn button_to_lwjgl(button: MouseButton) -> i32 {
	match button {
		MouseButton::Left => 1,
		MouseButton::Middle => 3,
		MouseButton::Right => 2,
	}
}
