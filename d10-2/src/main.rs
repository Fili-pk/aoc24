use itertools::Itertools;
use std::{
	cell::RefCell,
	collections::HashMap,
	fs::File,
	io::{BufRead, BufReader},
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
	thread::scope,
};
use utils::*;

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let lines: Vec<_> = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
			x.chars()
				.map(|x| String::from(x).parse::<u8>().unwrap_or(255))
				.collect::<Vec<_>>()
		})
		.collect();

	let mut scr = 0;
	for i in 0..lines.len() {
		for j in 0..lines[0].len() {
			let disc = RefCell::new(HashMap::new()); //used hashmap because i miss understood task
			rec(0, &disc, &lines, i as i32, j as i32);
			println!("{:#?}", disc.borrow());
			scr += disc.borrow().iter().map(|x| *x.1).sum::<u64>();
		}
	}

	println!("{scr}")
}

fn rec(st: u8, scr: &RefCell<HashMap<(i32, i32), u64>>, lines: &Vec<Vec<u8>>, i: i32, j: i32) {
	if !lines
		.geti(i)
		.and_then(|x| x.geti(j))
		.is_some_and(|x| *x == st)
	{
		return;
	}

	if st == 9 {
		scr.borrow_mut()
			.entry((i, j))
			.and_modify(|x| *x += 1)
			.or_insert(1);

		return;
	}

	rec(st + 1, scr, lines, i + 1, j);
	rec(st + 1, scr, lines, i, j + 1);
	rec(st + 1, scr, lines, i - 1, j);
	rec(st + 1, scr, lines, i, j - 1)
}
