
//
//  Configuration
//! Create and update emulator configuration.
//


use serialize::json;
use serialize::json::Json;
use std::io::fs::File;


macro_rules! get(
	($tree:ident, $name:expr, $method:ident) => (
		{
			let value = try!($tree.get(&($name.to_string()))
				.ok_or(format!("Configuration option `{}` doesn't exist.", $name)));
			try!(value.$method()
				.ok_or(format!("Option `{}` is of an incorrect type.", $name)))
		}
	)
);


/// The default configuration file's contents.
pub const DEFAULT: &'static str = include_str!("config/default.json");


/// A set of configuration options.
#[derive(Show, Clone)]
pub struct Config {
	pub computer_width: u32,
	pub computer_height: u32,
	pub pocket_width: u32,
	pub pocket_height: u32,
	pub space_limit: u64,
	pub border_width: u32,
	pub border_height: u32,
}


impl Config {

	/// Load the configuration from a file.
	pub fn from_file(path: &Path) -> Result<Config, String> {
		let mut file = File::open(path);
		let contents = try!(file.read_to_end().ok()
			.ok_or("Failed to read configuration file.".to_string()));
		let string = try!(String::from_utf8(contents).ok()
			.ok_or("Configuration file not valid UTF-8.".to_string()));
		let json = json::from_str(string.as_slice());

		match json {
			Ok(decoded) => Config::from_json(decoded),
			Err(err) => Err(format!("Failed to decode JSON file: {:?}", err)),
		}
	}

	/// Load the configuration from a JSON object.
	fn from_json(json: Json) -> Result<Config, String> {
		let tree = try!(json.as_object()
			.ok_or("Root JSON object is not a dictionary.".to_string()));

		Ok(Config {
			computer_width: get!(tree, "computer width", as_u64) as u32,
			computer_height: get!(tree, "computer height", as_u64) as u32,
			pocket_width: get!(tree, "pocket width", as_u64) as u32,
			pocket_height: get!(tree, "pocket height", as_u64) as u32,
			space_limit: get!(tree, "space limit", as_u64),
			border_width: get!(tree, "border width", as_u64) as u32,
			border_height: get!(tree, "border height", as_u64) as u32,
		})
	}

}
