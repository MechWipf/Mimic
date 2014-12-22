
//
//  Configuration
//! Create and update emulator configuration.
//


/// Returns the default configuration file contents.
pub fn default() -> &'static str {
	include_str!("default_config.json")
}
