#![allow(dead_code)]

use crate::man::man::Manager;

mod io;
mod man;

fn main() {
	Manager::init().start();
}
