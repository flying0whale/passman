use std::fs::{File, OpenOptions};
use std::io::{Write, stdout, stdin, Read};
use std::path::Path;
use serde_json::Value;

pub fn print(text: String) {
	print!("{text}");

	stdout().flush().unwrap();
}

pub fn cls() {
	clearscreen::clear().expect("failed to clear screen");
}

// TODO: return Result instead of plain string
pub fn read_string() -> String {
	let mut input = String::new();

	stdin().read_line(&mut input).unwrap();

	return input.trim().parse().unwrap();
}

pub fn pad_right(text: &str, width: usize) -> String {
	format!("{:width$}", text, width=width)
}

pub fn file_exists(path: &str) -> bool {
	Path::new(path).exists()
}

pub fn read_file(path: String) -> Result<String, String> {
	let mut content: String = String::new();

	if !file_exists(path.as_str()) {
		return Err(format!("Could not open file {}: File doesn't exist", path))
	}

	let mut file = File::open(path.clone()).unwrap();

	return match file.read_to_string(&mut content) {
		Ok(_) => { Ok(content) }
		Err(_) => { Err(format!("Could not read file {}", path)) }
	};
}

pub fn rewrite_file(path: String, content: Value) -> Result<(), String> {
	let mut file = match OpenOptions::new().write(true).truncate(true).open(path.clone()) {
		Ok(f) => { f }
		Err(_) => { return Err(format!("Could not open file {}", path)) }
	};

	return match file.write_all(content.to_string().as_ref()) {
		Ok(_) => { Ok(()) }
		Err(_) => { return Err(format!("Could not write to file {}", path)); }
	};
}

pub fn create_file(path: &str) -> Result<File, String> {
	if file_exists(path) {
		return Err(format!("Could not create file {}: File already exists", path));
	}

	match File::create(path) {
		Ok(file) => Ok(file),
		Err(_) => Err(format!("Could not create file {}", path))
	}
}