use itertools::Itertools;
use std::{
	collections::HashMap,
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let mut c1 = HashMap::new();
	let mut c2 = HashMap::new();
	for x in file.lines().map(|x| {
		let x = x.unwrap();
		let x = x.split_once("   ").unwrap();
		(x.0.parse::<i32>().unwrap(), x.1.parse::<i32>().unwrap())
	}) {
		c1.entry(x.0).and_modify(|x| *x += 1).or_insert(1);
		c2.entry(x.1).and_modify(|x| *x += 1).or_insert(1);
	}

	let scr = c1
		.iter()
		.map(|(k, v)| *c2.get(k).unwrap_or(&0) * v * *k)
		.sum::<i32>();

	println!("{}", scr);
}
