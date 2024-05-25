use crate::man::man::Manager;

mod io;
mod man;
mod storage;

fn main() {
	match Manager::init() {
		Ok(mut m) => {
			m.start();
		}
		Err(str) => { println!("{}", str) }
	};
}
