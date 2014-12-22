
//
//  Configuration
//! Create and update emulator configuration.
//


extern crate serialize;

use self::serialize::json;
use self::serialize::json::Json;
use std::io::fs::File;


macro_rules! get(
	($tree:ident, $name:expr, $method:ident) => (
		$tree.get(&($name.to_string()))
			.expect(format!("Configuration option `{}` doesn't exist.", $name).as_slice())
			.$method()
			.expect(format!("Option `{}` is of an incorrect type.", $name).as_slice());
	)
);


/// The default configuration file contents.
pub const DEFAULT: &'static str = include_str!("default_config.json");


/// A set of configuration options.
#[deriving(Show, Clone)]
pub struct Config {
	pub computer_width: u32,
	pub computer_height: u32,
	pub pocket_width: u32,
	pub pocket_height: u32,
	pub enable_http: bool,
	pub space_limit: u64,
}


impl Config {

	/// Load the configuration from a file.
	pub fn from_file(path: &Path) -> Config {
		let mut file = File::open(path);
		let contents = file.read_to_end().ok()
			.expect("Could not read configuration file.");
		let string = String::from_utf8(contents).ok()
			.expect("Configuration file is not UTF-8.");
		let json = json::from_str(string.as_slice());

		match json {
			Ok(decoded) => Config::from_json(decoded),
			Err(_) => panic!("Failed to load configuration"),
		}
	}

	/// Load the configuration from a JSON object.
	fn from_json(json: Json) -> Config {
		let tree = json.as_object()
			.expect("Failed to load configuration. Root object is not a dictionary.");

		Config {
			computer_width: get!(tree, "computer width", as_u64) as u32,
			computer_height: get!(tree, "computer height", as_u64) as u32,
			pocket_width: get!(tree, "pocket width", as_u64) as u32,
			pocket_height: get!(tree, "pocket height", as_u64) as u32,
			enable_http: get!(tree, "enable http", as_boolean),
			space_limit: get!(tree, "space limit", as_u64),
		}
	}

}
