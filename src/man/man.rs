use crate::io::io::{pad_right, print, read_string};
use crate::man::store::Record;

pub struct Manager {
	next_id: i8,
	records: Vec<Record>,
	run: bool
}

impl Manager {
	pub fn init() -> Manager {
		Manager {
			next_id: 1,
			records: Vec::new(),
			run: false
		}
	}

	pub fn start(&mut self) {
		self.run = true;
		println!("Welcome!");

		Self::print_help();

		while self.run {
			let cmd = Self::read_command();

			match cmd.title.as_str() {
				"help" => Self::print_help(),
				"list" => Self::print_records(self),
				"exit" => self.run = false,
				"add"  => {
					let title = cmd.args.iter().nth(0).unwrap().clone();
					let pass  = cmd.args.iter().nth(1).unwrap().clone();
					let login = cmd.args.iter().nth(2).unwrap().clone();

					let record = Record::create(title, pass, login);
					self.add_record(record);

					self.print_records();
				},
				_ => println!("This is not a command. Something's wrong")
			}
		}
	}

	fn print_records(&self) {
		for rec in &self.records {
			println!("\n{}", *rec);
		}
	}

	fn add_record(&mut self, mut record: Record) {
		record.id = self.next_id;
		self.next_id += 1;

		self.records.push(record);
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
		println!("{}shows all commands", pad_right("help", WIDTH));
		println!("{}shows all records",  pad_right("list", WIDTH));
		println!("{}adds new record",    pad_right("add [title] [password] [login]", WIDTH));
		println!("{}exits program",      pad_right("exit", WIDTH));
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