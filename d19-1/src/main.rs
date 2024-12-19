use itertools::Itertools;
use std::{
	collections::VecDeque,
	fs::File,
	io::{BufRead, BufReader},
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
	thread::scope,
};
use utils::*;

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let mut lines = file.lines();
	let mut pat = lines
		.next()
		.unwrap()
		.unwrap()
		.split(", ")
		.map(|x| x.chars().collect::<Vec<_>>())
		.collect::<Vec<_>>();
	lines.next();
	let wrd: Vec<_> = lines
		.map(|x| {
			let x = x.unwrap();
			x.chars().collect::<Vec<_>>()
		})
		.collect();

	let mut scr = 0;
	for wrd in wrd {
		if is_good(&wrd, &pat) {
			scr += 1;
		}
	}

	println!("{scr}")
}

fn is_good(wrd: &[char], pat: &[Vec<char>]) -> bool {
	if wrd.is_empty() {
		return true;
	}

	for x in pat {
		if wrd.starts_with(x) && is_good(&wrd[x.len()..], pat) {
			return true;
		}
	}

	false
}
