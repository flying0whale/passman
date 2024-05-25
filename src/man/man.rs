use std::str::FromStr;
use std::string::ToString;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use crate::io::io::{cls, create_file, file_exists, pad_right, print, read_file, read_string, rewrite_file};
use crate::man::store::Record;

const PATH: &str = "json_data.json";

#[derive(Serialize, Deserialize)]
pub struct Manager {
	access_pass: Option<String>,
	next_id: i8,
	records: Vec<Record>,
	run: bool
}

impl Manager {
	pub fn init() -> Manager {
		Manager {
			access_pass: None,
			next_id: 1,
			records: Vec::new(),
			run: false
		}
	}

	pub fn start(&mut self) {
		match self.load() {
			Ok(_) => {
				if self.access_pass.is_some() {
					loop {
						print("Enter a password >> ".to_string());
						let pass = read_string();
						if pass == self.access_pass.clone().unwrap() { cls(); break; }

						println!("Wrong password");
					}
				} else {
					println!("You don't have an access password for a passman. Are you want to create one?");

					loop {
						print("y/n >> ".to_string());

						match read_string().as_str() {
							"y" => {
								print("Enter a password >> ".to_string());
								let pass = read_string();
								self.access_pass = Some(pass);
								self.save().expect("Failed to save password D:");

								cls();
								break;
							},
							"n" => {
								cls();
								break;
							},
							_ => { println!("That's not an answer. Do you want to create a password?") }
						};
					}
				}
			}
			Err(err) => {
				println!("{}", err);
				return;
			}
		};

		self.run = true;
		println!("Welcome!");
		Self::print_help();

		while self.run {
			let cmd = Self::read_command();
			cls();
			match cmd.title.as_str() {
				"help" => Self::print_help(),
				"list" => Self::print_records(&self.records),
				"exit" => self.run = false,
				"add"  => {
					self.add_record(cmd.args);
					self.save().unwrap();
					Self::print_records(&self.records);
				},
				"remove" => {
					self.remove_record(cmd.args);
					self.save().unwrap();
					Self::print_records(&self.records);
				},
				"find" => {
					let records = self.find_record(cmd.args);
					Self::print_records(&records);
				},
				"cls" => { cls() }
				_ => println!("This is not a command. Something's wrong")
			}
		}
	}

	fn load(&mut self) -> Result<(), String> {
		if !file_exists(PATH) {
			match create_file(PATH) {
				Ok(_) => {  }
				Err(err) => { return Err(err) }
			};
		}

		let json = match read_file(PATH) {
			Ok(content) => { content }
			Err(err) => { return Err(err) }
		};

		if json.is_empty() {
			return Ok(());
		}

		let man: Manager = match from_str(&*json) {
			Ok(m) => { m },
			Err(_) => { return Err(format!("Could not read data from JSON file {}", PATH)) }
		};

		self.access_pass = man.access_pass;
		self.next_id = man.next_id;
		self.records = man.records;

		return Ok(());
	}

	fn save(&self) -> Result<(), String> {
		let json = to_string(self).unwrap();

		return match rewrite_file(PATH, json) {
			Ok(_) => { Ok(()) }
			Err(err) => { Err(err) }
		};
	}

	fn print_records(records: &Vec<Record>) {
		for rec in records {
			println!("\n{}", *rec);
		}
	}

	fn find_record(&self, args: Vec<String>) -> Vec<Record> {
		let title = args.first().unwrap();
		return self.records.iter().filter(|r| r.title.contains(title)).map(|r| r.clone()).collect();
	}

	fn remove_record(&mut self, args: Vec<String>) {
		let id = i8::from_str(args.first().unwrap()).unwrap();

		let pos = self.records.iter().position(|r| r.id == id);
		if pos.is_none() {
			println!("Can not find a record with id {}", id);
			return;
		}

		self.records.remove(pos.unwrap());
	}

	fn add_record(&mut self, args: Vec<String>) {
		let title = args.iter().nth(0).unwrap().clone();
		let login = args.iter().nth(1).unwrap().clone();
		let pass  = args.iter().nth(2).unwrap().clone();

		let mut record = Record::create(title, pass, login);
		record.id = self.next_id;
		self.next_id += 1;

		self.records.push(record);
		self.save().unwrap();
	}

	fn read_command() -> Command {
		print("\nEnter a command >> ".parse().unwrap());
		let input: Vec<String> = read_string().trim().split(' ')
													 .map(|x| x.to_string())
													 .collect();
		let title = input.first().unwrap().to_lowercase();

		let mut args: Vec<String> = Vec::new();

		if input.iter().count() > 1 {
			args = input[1..input.iter().count()].to_owned();
		}

		return Command::new(title.clone(), args);
	}

	fn print_help () {
		const WIDTH: usize = 50;

		println!("available commands");
		println!("{}shows all commands",             pad_right("help", WIDTH));
		println!("{}shows all records",              pad_right("list", WIDTH));
		println!("{}adds new record",                pad_right("add [title] [login] [password]", WIDTH));
		println!("{}deletes a record",               pad_right("remove [id]", WIDTH));
		println!("{}finds a record by it's title",   pad_right("find [title]", WIDTH));
		println!("{}clears a console",               pad_right("cls", WIDTH));
		println!("{}exits program",                  pad_right("exit", WIDTH));
	}
}

pub struct Command {
	title: String,
	args: Vec<String>
}

impl Command {
	pub fn new(title: String, args: Vec<String>) -> Command {
		Command { title, args }
	}
}