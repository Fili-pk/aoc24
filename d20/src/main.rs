use itertools::Itertools;
use std::{
	borrow::BorrowMut,
	cell::RefCell,
	collections::{HashMap, HashSet},
	fs::File,
	io::{BufRead, BufReader},
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
	sync::Mutex,
	thread::scope,
	u32,
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

	let mut x_s = 0;
	let mut y_s = 0;
	's: for (i, x) in lines.iter_mut().enumerate() {
		for (j, x) in x.iter_mut().enumerate() {
			if *x == 'S' {
				x_s = j;
				y_s = i;
				*x = '.';
				break 's;
			}
		}
	}

	let mut cache = HashMap::new();
	let mut v = vec![vec![false; lines[0].len()]; lines.len()];
	*WITHOUT_CHEAT.lock().unwrap() =
		path(&mut v, x_s as i32, y_s as i32, &lines, &mut cache).unwrap() as u16;
	println!("{:?}", *WITHOUT_CHEAT.lock().unwrap());

	let mut v = vec![vec![false; lines[0].len()]; lines.len()];
	let mut cheats = HashSet::new();

	path_cheat(
		&mut v,
		x_s as i16,
		y_s as i16,
		None,
		&mut cheats,
		0,
		&lines,
		&mut cache,
	);

	println!("{:#?}", cheats.len());
}

static WITHOUT_CHEAT: Mutex<u16> = Mutex::new(0);
fn path_cheat(
	v: &mut [Vec<bool>],
	x: i16,
	y: i16,
	mut cheat: Option<((i16, i16), (i16, i16))>,
	cheats: &mut HashSet<((i16, i16), (i16, i16))>,
	scr: u16,
	lines: &[Vec<char>],
	cache: &mut HashMap<(i16, i16), u16>,
) -> Option<u16> {
	let Some(c) = lines.geti2(y as i32, x as i32) else {
		return None;
	};

	if scr > 9217 {
		return None;
	}

	if v[y as usize][x as usize] {
		return None;
	}

	if *c == 'E' {
		if scr + 100 <= *WITHOUT_CHEAT.lock().unwrap() {
			cheats.insert(cheat.unwrap());
		}

		return Some(0);
	}

	if cheat.is_some() {
		if let Some(v) = cache.get(&(x, y)) {
			if scr + *v + 100 <= *WITHOUT_CHEAT.lock().unwrap() {
				cheats.insert(cheat.unwrap());
			}

			return Some(*v);
		}
	}

	if *c == '#' {
		return None;
	}

	if cheat.is_none() {
		for i in -20..=20 {
			let d: i16 = i16::abs(i);
			for j in (-20 + d)..=(20 - d) {
				if i == 0 && j == 0 {
					continue;
				}

				path_cheat(
					v,
					x + j,
					y + i,
					Some(((x, y), (x + j, y + i))),
					cheats,
					scr + d as u16 + j.abs() as u16,
					lines,
					cache,
				);
			}
		}
	}

	v[y as usize][x as usize] = true;

	let mut b = path_cheat(v, x + 1, y, cheat, cheats, scr + 1, lines, cache);
	let a = path_cheat(v, x - 1, y, cheat, cheats, scr + 1, lines, cache);
	if a.is_some_and(|x| x <= b.unwrap_or(0)) {
		b = a;
	}
	let a = path_cheat(v, x, y + 1, cheat, cheats, scr + 1, lines, cache);
	if a.is_some_and(|x| x <= b.unwrap_or(0)) {
		b = a;
	}
	let a = path_cheat(v, x, y - 1, cheat, cheats, scr + 1, lines, cache);
	if a.is_some_and(|x| x <= b.unwrap_or(0)) {
		b = a;
	}

	v[y as usize][x as usize] = false;

	if let Some(b) = &mut b {
		*b += 1;
		if cheat.is_some() && *c != '#' {
			cache.insert((x, y), *b);
		}
	}

	b
}

fn path(
	v: &mut [Vec<bool>],
	x: i32,
	y: i32,
	lines: &[Vec<char>],
	cache: &mut HashMap<(i16, i16), u16>,
) -> Option<u32> {
	let Some(c) = lines.geti2(y, x) else {
		return None;
	};

	if v[y as usize][x as usize] {
		return None;
	}

	if *c == 'E' {
		return Some(0);
	}

	if let Some(b) = cache.get(&(x as i16, y as i16)) {
		return Some(*b as u32);
	}

	if *c == '#' {
		return None;
	}

	v[y as usize][x as usize] = true;

	let mut b = path(v, x + 1, y, lines, cache);
	let a = path(v, x - 1, y, lines, cache);
	if a.is_some_and(|x| x <= b.unwrap_or(u32::MAX)) {
		b = a;
	}
	let a = path(v, x, y + 1, lines, cache);
	if a.is_some_and(|x| x <= b.unwrap_or(u32::MAX)) {
		b = a;
	}
	let a = path(v, x, y - 1, lines, cache);
	if a.is_some_and(|x| x <= b.unwrap_or(u32::MAX)) {
		b = a;
	}

	v[y as usize][x as usize] = false;

	if let Some(b) = &mut b {
		*b += 1;
		cache.insert((x as i16, y as i16), *b as u16);
	}
	v[y as usize][x as usize] = true;
	b
}

trait GetI2Ext<T> {
	fn geti2(&self, y: i32, x: i32) -> Option<&T>;
	fn geti2_mut(&mut self, y: i32, x: i32) -> Option<&mut T>;
}

impl<T> GetI2Ext<T> for [Vec<T>] {
	fn geti2(&self, y: i32, x: i32) -> Option<&T> {
		self.geti(y).and_then(|d| d.geti(x))
	}

	fn geti2_mut(&mut self, y: i32, x: i32) -> Option<&mut T> {
		self.geti_mut(y).and_then(|d| d.geti_mut(x))
	}
}
