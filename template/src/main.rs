use itertools::Itertools;
use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	file.lines().map(|x| {
		let x = x.unwrap();
	})
}
