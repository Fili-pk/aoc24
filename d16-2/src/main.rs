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
	let mut vst = vec![vec![0; lines[0].len()]; lines.len()];
	let mut scr = vec![vec![i32::MAX; lines[0].len()]; lines.len()];
	q(&lines, &mut vst, min_scr, &mut scr, Dir::R, 0, x, y);
	for x in vst.iter() {
		for x in x {
			print!("{x}")
		}

		println!()
	}
	let d: usize = vst
		.iter()
		.flat_map(|x| x.iter())
		.filter(|x| **x != 0)
		.count();

	println!("{}", d)
}

fn q(
	lines: &Vec<Vec<char>>,
	vst: &mut Vec<Vec<i32>>,
	max: i32,
	scr2: &mut Vec<Vec<i32>>,
	dir: Dir,
	mut scr: i32,
	x: usize,
	y: usize,
) -> bool {
	if lines[y][x] == '#' {
		return false;
	}

	if max < scr {
		return false;
	}

	if scr2[y][x] < scr {
		return false;
	}

	scr2[y][x] = scr + 1000;

	vst[y][x] += 1;

	if lines[y][x] == 'E' {
		return true;
	}

	scr += 1;
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

	let b = q(lines, vst, max, scr2, Dir::U, scr + u, x, y - 1);
	let b = q(lines, vst, max, scr2, Dir::D, scr + d, x, y + 1) || b;
	let b = q(lines, vst, max, scr2, Dir::R, scr + r, x + 1, y) || b;
	let b = q(lines, vst, max, scr2, Dir::L, scr + l, x - 1, y) || b;

	if !b {
		vst[y][x] -= 1;
	}

	b
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
