#![allow(dead_code)]

use crate::man::man::Manager;

mod io;
mod man;

fn main() {
	let mut man = Manager::init();
	man.start();
}
