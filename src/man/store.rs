use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use crate::io::io::pad_right;

#[derive(Serialize, Deserialize, Clone)]
pub struct Record {
	pub id: i8,
	pub title: String,
	pub login: String,
	pub pass: String
}

impl Record {
	pub fn create(title: String, pass: String, login: String) -> Record {
		Record {
			id: 0,
			title,
			login,
			pass
		}
	}
}

impl Display for Record {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}. {}\n", self.id, self.title).expect("TODO: panic message");
		write!(f, "{} {}", pad_right(&*self.login, 25), self.pass)
	}
}