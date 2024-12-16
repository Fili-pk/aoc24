use itertools::Itertools;
use std::{
	cell::RefCell,
	fs::File,
	i32,
	io::{BufRead, BufReader},
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
	thread::scope,
	usize,
};
use utils::*;

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let mut lines: Vec<_> = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
			x.chars().collect::<Vec<_>>()
		})
		.collect();

	let mut x = 0;
	let mut y = 0;
	let mut x_end = 0;
	let mut y_end = 0;
	for (i, d) in lines.iter_mut().enumerate() {
		for (j, d) in d.iter_mut().enumerate() {
			if *d == 'S' {
				x = j;
				y = i;
				*d = '.';
			}

			if *d == 'E' {
				x_end = j;
				y_end = i;
			}
		}
	}

	let mut scr = vec![vec![i32::MAX; lines[0].len()]; lines.len()];
	let mut min_scr = i32::MAX;
	find(&lines, &mut scr, Dir::R, 0, &mut min_scr, x, y);
	println!("{}", min_scr)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
	R,
	U,
	D,
	L,
}

fn find(
	lines: &Vec<Vec<char>>,
	scr: &mut Vec<Vec<i32>>,
	dir: Dir,
	mut scrr: i32,
	min_scr: &mut i32,
	x: usize,
	y: usize,
) {
	if lines[y][x] == '#' {
		return;
	}

	if scr[y][x] <= scrr {
		return;
	}

	scr[y][x] = scrr;

	if lines[y][x] == 'E' {
		if scrr < *min_scr {
			*min_scr = scrr;
		}

		return;
	}

	scrr += 1;
	let u = match dir {
		Dir::L => 1000,
		Dir::R => 1000,
		Dir::D => 2000,
		Dir::U => 0,
	};

	let d = match dir {
		Dir::L => 1000,
		Dir::R => 1000,
		Dir::D => 0,
		Dir::U => 2000,
	};

	let l = match dir {
		Dir::L => 0,
		Dir::R => 2000,
		Dir::D => 1000,
		Dir::U => 1000,
	};

	let r = match dir {
		Dir::L => 2000,
		Dir::R => 0,
		Dir::D => 1000,
		Dir::U => 1000,
	};

	find(lines, scr, Dir::U, scrr + u, min_scr, x, y - 1);
	find(lines, scr, Dir::D, scrr + d, min_scr, x, y + 1);
	find(lines, scr, Dir::R, scrr + r, min_scr, x + 1, y);
	find(lines, scr, Dir::L, scrr + l, min_scr, x - 1, y);
}
