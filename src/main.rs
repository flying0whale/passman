use crate::man::man::Manager;

mod io;
mod man;
mod storage;

fn main() {
	match Manager::init() {
		Ok(mut man) => {
			man.start();
		}
		Err(str) => { println!("{}", str) }
	};
}
