use itertools::Itertools;
use std::{
	cell::RefCell,
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
			let disc = RefCell::new(vec![vec![false; lines[0].len()]; lines.len()]);
			let a = rec(0, &disc, &lines, i as i32, j as i32);
			println!("{a}");
			scr += a;
		}
	}

	println!("{scr}")
}

fn rec(st: u8, disc: &RefCell<Vec<Vec<bool>>>, lines: &Vec<Vec<u8>>, i: i32, j: i32) -> u64 {
	if !lines
		.geti(i)
		.and_then(|x| x.geti(j))
		.is_some_and(|x| *x == st)
	{
		return 0;
	}

	if st == 9 {
		if !disc.borrow()[i as usize][j as usize] {
			disc.borrow_mut()[i as usize][j as usize] = true;
			return 1;
		}
		return 0;
	}

	rec(st + 1, disc, lines, i + 1, j)
		+ rec(st + 1, disc, lines, i, j + 1)
		+ rec(st + 1, disc, lines, i - 1, j)
		+ rec(st + 1, disc, lines, i, j - 1)
}
