use std::fs::{File, OpenOptions};
use std::io::{Write, stdout, stdin, Read};
use std::path::Path;

pub fn print(text: String) {
	print!("{text}");

	stdout().flush().unwrap();
}

pub fn cls() {
	clearscreen::clear().expect("failed to clear screen");
}

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

pub fn rewrite_file(path: &str, content: String) -> Result<(), String> {
	let err = &*format!("Could not write to file {}", path);
	let mut file_opt: Option<File> = None;

	if !file_exists(path) {
		match create_file(path) {
			Ok(f) => { file_opt = Some(f) }
			Err(err) => { return Err(err) }
		}
	}

	if file_opt.is_none() {
		file_opt = Some(OpenOptions::new().write(true).truncate(true).open(path).unwrap());
	}

	if let Some(mut file) = file_opt {
		return match file.write_all(content.as_ref()) {
			Ok(_) => { Ok(()) }
			Err(_) => { Err(err.parse().unwrap()) }
		}
	}

	return Err(err.parse().unwrap());
}

pub fn read_file(path: &str) -> Result<String, String> {
	let err = &*format!("Could not read file {}", path);
	if !file_exists(path) {
		return Err(err.to_string());
	}
	
	let mut file = File::open(path).unwrap();
	let mut content = String::new();
	
	return match file.read_to_string(&mut content) {
		Ok(_)  => { Ok(content) }
		Err(_) => { Err(err.parse().unwrap()) }
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