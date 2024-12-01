use itertools::Itertools;
use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let (mut l1, mut l2): (Vec<i32>, Vec<i32>) = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
			let x = x.split_once("   ").unwrap();
			(x.0.parse::<i32>().unwrap(), x.1.parse::<i32>().unwrap())
		})
		.collect();

	l1.sort();
	l2.sort();
	println!(
		"{}",
		l1.iter()
			.zip(l2.iter())
			.map(|x| (x.0 - x.1).abs())
			.sum::<i32>()
	)
}
