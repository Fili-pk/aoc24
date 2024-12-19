use itertools::Itertools;
use std::{
	collections::{HashMap, VecDeque},
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
	let mut hash = HashMap::new();
	for (i, wrd) in wrd.iter().enumerate() {
		println!("{i}");
		scr += good(wrd, &pat, &mut hash);
	}

	println!("{scr}")
}

fn good<'a>(wrd: &'a [char], pat: &[Vec<char>], hash: &mut HashMap<&'a [char], u64>) -> u64 {
	if wrd.is_empty() {
		return 1;
	}

	if let Some(w) = hash.get(wrd) {
		return *w;
	}

	let mut sum = 0;
	for x in pat {
		if wrd.starts_with(x) {
			sum += good(&wrd[x.len()..], pat, hash);
		}
	}

	hash.insert(wrd, sum);

	sum
}
