use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use crate::io::io::{create_file, file_exists, read_file, rewrite_file};
use crate::storage::record::Record;

#[derive(Serialize, Deserialize)]
pub struct SaveData {
	pub next_id: i8,
	pub pass: Option<String>,
	pub records: Vec<Record>
}

pub struct Storage {
	path: String
}

impl Storage {
	pub fn connect(path: &str) -> Result<Storage, String> {
		match Self::init_file(path) {
			Ok(_) => { }
			Err(str) => { return Err(str) }
		}

		return Ok(Storage {
			path: path.to_string()
		});
	}

	fn init_file(path: &str) -> Result<(), String> {
		if !file_exists(path) {
			match create_file(path) {
				Ok(_) => { },
				Err(err) => return Err(err)
			};
		}

		return Ok(());
	}

	pub fn save(&mut self, data: SaveData) -> Result<(), String> {
		let pass = data.pass;
		let id = data.next_id;
		let recs = data.records;

		let json = json!({
			"pass": pass,
			"next_id": id,
			"records": recs
		});

		return match rewrite_file(self.path.to_string(), json) {
			Ok(_) => { Ok(()) }
			Err(err) => Err(err)
		};
	}

	pub fn load_data(&mut self) -> Result<SaveData, String> {
		let json = match read_file(self.path.to_string()) {
			Ok(res) => res,
			Err(err) => return Err(err)
		};

		if json.is_empty() {
			return Ok(SaveData {
				next_id: 1,
				records: vec![],
				pass: None
			});
		}

		return match from_str(&*json) {
			Ok(res) => { Ok(res) },
			Err(_) => { Err(format!("Could not deserialize JSON value: {}", json)) }
		};
	}
}