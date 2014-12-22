
//
//  Storage
//! Manages path finding.
//


use std::os;
use std::io;
use std::io::fs;
use std::io::fs::{File, PathExtensions};

use config;


/// The name of the storage directory.
const STORAGE_DIR_NAME: &'static str = "mimic";

/// The name of the configuration file.
const CONFIG_FILE_NAME: &'static str = "config.json";

/// The name of the ComputerCraft jar file.
const JAR_FILE_NAME: &'static str = "computercraft.jar";


/// Creates the storage directory and default configuration file, if one doesn't exist.
pub fn create() {
	// Storage directory
	let storage_dir = storage();
	if !storage_dir.exists() {
		fs::mkdir(&storage_dir, io::USER_RWX).unwrap();
	}

	// Configuration file
	let config_path = config();
	if !config_path.exists() {
		let mut file = File::create(&config_path);
		file.write_str(config::DEFAULT).unwrap();
	}
}

/// Returns the path to the home directory.
pub fn home() -> Path {
	let os_name = os::consts::SYSNAME;
	if os_name == "macos" {
		os::homedir().expect("Could not get home directory")
	} else {
		panic!("Using unsupported OS: {}", os_name);
	}
}

/// Returns the path to the resources folder.
pub fn resources() -> Path {
	let mut exe_path = os::self_exe_path().expect("Could not get path to executable.");
	let os_name = os::consts::SYSNAME;
	if os_name == "macos" {
		exe_path.push_many(&["..", "Resources"]);
		exe_path
	} else {
		panic!("Using unsupported OS: {}", os_name);
	}
}

/// Returns the path to the storage directory.
pub fn storage() -> Path {
	let mut dir = home();
	dir.push_many(&["Library", "Application Support", STORAGE_DIR_NAME]);
	dir
}

/// Returns the path to the configuration file.
pub fn config() -> Path {
	let mut path = storage();
	path.push(CONFIG_FILE_NAME);
	path
}

/// Returns the classpath for the Java VM.
pub fn classpath() -> Vec<Path> {
	let dir = resources();
	let mut jar_file = dir.clone();
	jar_file.push(JAR_FILE_NAME);
	vec![dir, jar_file]
}
