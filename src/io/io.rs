use std::io::{Write, stdout, stdin };

pub fn print(text: String) {
	print!("{text}");

	stdout().flush().unwrap();
}

pub fn read_string() -> String {
	let mut input = String::new();

	stdin().read_line(&mut input).unwrap();

	return input.trim().parse().unwrap();
}

pub fn pad_right(text: &str, width: usize) -> String {
	format!("{:width$}", text, width=width)
}