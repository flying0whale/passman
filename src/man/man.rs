use std::str::FromStr;
use std::string::ToString;
use crate::io::io::{cls, pad_right, print, read_string};
use crate::storage::record::Record;
use crate::storage::storage::{SaveData, Storage};

const PATH: &str = "json_data.json";

pub struct Manager {
	access_pass: Option<String>,
	next_id: i8,
	records: Vec<Record>,
	run: bool,
	storage: Storage
}

impl Manager {
	pub fn init() -> Result<Manager, String> {
		let storage = match Storage::connect(PATH) {
			Ok(res) => { res }
			Err(str) => { return Err(str) }
		};

		let mut man = Manager {
			access_pass: None,
			next_id: 0,
			records: vec![],
			run: false,
			storage
		};

		match man.load() {
			Ok(_) => { }
			Err(str) => { return Err(str) }
		};

		return Ok(man);
	}

	pub fn start(&mut self) {
		match self.load() {
			Ok(_) => {
				if self.access_pass.is_some() {
					loop {
						print("Enter a password >> ".to_string());
						let pass = read_string().unwrap();
						if pass == self.access_pass.clone().unwrap() { cls(); break; }

						println!("Wrong password");
					}
				} else {
					println!("You don't have an access password for a passman. Are you want to create one?");

					loop {
						print("y/n >> ".to_string());

						match read_string().unwrap().as_str() {
							"y" => {
								print("Enter a password >> ".to_string());
								let pass = read_string().unwrap();
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
					if self.add_record(cmd.args) {
						self.save().unwrap();
						Self::print_records(&self.records);
					}
				},
				"remove" => {
					if self.remove_record(cmd.args) {
						self.save().unwrap();
						Self::print_records(&self.records);
					}
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

	pub fn get_data(&self) -> SaveData {
		return SaveData {
			next_id: self.next_id,
			records: self.records.clone(),
			pass: self.access_pass.clone()
		};
	}

	fn load(&mut self) -> Result<(), String> {
		let data = match self.storage.load_data() {
			Ok(d) => { d }
			Err(err) => { return Err(err) }
		};

		self.next_id = data.next_id;
		self.records = data.records;
		self.access_pass = data.pass;

		return Ok(());
	}

	fn save(&mut self) -> Result<(), String> {
		return match self.storage.save(self.get_data()) {
			Ok(_) => { Ok(()) }
			Err(str) => { Err(str) }
		}
	}

	fn print_records(records: &Vec<Record>) {
		for rec in records {
			println!("\n{}", *rec);
		}
	}

	fn find_record(&self, args: Vec<String>) -> Vec<Record> {
		if args.iter().count() != 0 {
			println!("Wrong arguments");
			return vec![];
		}

		let title = args.first().unwrap();
		return self.records.iter().filter(|r| r.title.contains(title)).map(|r| r.clone()).collect();
	}

	fn remove_record(&mut self, args: Vec<String>) -> bool {
		if args.iter().count() != 1 {
			println!("Wrong arguments");
			return false;
		}

		let id = match i8::from_str(args.first().unwrap()) {
			Ok(num) => { num }
			Err(err) => {
				println!("{}", err);
				return false;
			}
		};

		let pos = self.records.iter().position(|r| r.id == id);
		if pos.is_none() {
			println!("Can not find a record with id {}", id);
			return false;
		}

		self.records.remove(pos.unwrap());

		return true;
	}

	fn add_record(&mut self, args: Vec<String>) -> bool {
		if args.iter().count() != 3 {
			println!("Wrong arguments");
			return false;
		}

		let title = args.iter().nth(0).unwrap().clone();
		let login = args.iter().nth(1).unwrap().clone();
		let pass  = args.iter().nth(2).unwrap().clone();

		let mut record = Record::create(title, pass, login);
		record.id = self.next_id;
		self.next_id += 1;

		self.records.push(record);
		match self.save() {
			Ok(_) => { }
			Err(err) => {  println!("{}", err) }
		};

		return true;
	}

	fn read_command() -> Command {
		print("\nEnter a command >> ".parse().unwrap());
		let input: Vec<String> = read_string().unwrap().trim().split(' ')
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