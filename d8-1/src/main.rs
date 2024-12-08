use core::hash;
use itertools::Itertools;
use std::{
	collections::HashMap,
	fs::File,
	io::{BufRead, BufReader},
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
	thread::scope,
};

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let mut hash = HashMap::<char, Vec<(i32, i32)>>::new();
	let mut x_s = 0;
	let mut y_s = 0;
	file.lines().enumerate().for_each(|(i, x)| {
		let x = x.unwrap();
		for (j, x) in x.chars().enumerate() {
			x_s = j;
			y_s = i;
			hash.entry(x)
				.and_modify(|x| x.push((i as i32, j as i32)))
				.or_insert(vec![(i as i32, j as i32)]);
		}
	});

	let mut w = vec![vec![false; x_s + 1]; y_s + 1];
	for x in hash {
		if x.0 == '.' {
			continue;
		}

		for x in x.1.iter().combinations(2) {
			let diff_x = (x[0].1 - x[1].1).abs();
			let diff_y = (x[0].0 - x[1].0).abs();

			let x1;
			let x2;
			if x[0].1 < x[1].1 {
				x1 = x[0].1 - diff_x;
				x2 = x[1].1 + diff_x;
			} else {
				x1 = x[0].1 + diff_x;
				x2 = x[1].1 - diff_x;
			}

			let y1;
			let y2;
			if x[0].0 < x[1].0 {
				y1 = x[0].0 - diff_y;
				y2 = x[1].0 + diff_y;
			} else {
				y1 = x[0].0 + diff_y;
				y2 = x[1].0 - diff_y;
			}

			geti(&mut w, y1)
				.and_then(|x| geti(x, x1))
				.and_mut(|x| *x = true);
			geti(&mut w, y2)
				.and_then(|x| geti(x, x2))
				.and_mut(|x| *x = true);

			println!(
				"{},{} {},{} {},{} {},{} {},{}",
				x[0].0, x[0].1, x[1].0, x[1].1, diff_y, diff_x, x1, y1, x2, y2,
			);
			for x in w.iter() {
				for x in x {
					print!("{}", *x as u8)
				}

				println!()
			}
			println!()
		}
	}

	for x in w.iter() {
		for x in x {
			print!("{}", *x as u8)
		}

		println!()
	}

	println!("{}", w.iter().flatten().filter(|x| **x).count())
}

fn geti<T>(v: &mut [T], i: i32) -> Option<&mut T> {
	if i < 0 {
		return None;
	}

	v.get_mut(i as usize)
}
trait AndMutExt<T> {
	fn and_mut<F>(self, f: F)
	where
		F: FnOnce(&mut T);
}

impl<T> AndMutExt<T> for Option<&mut T> {
	fn and_mut<F>(self, f: F)
	where
		F: FnOnce(&mut T),
	{
		if let Some(x) = self {
			f(x)
		}
	}
}

fn tryy<'a, T: Send + 'a>(q: impl FnOnce() -> T + Send + 'a) -> Option<T> {
	scope(|s| s.spawn(q).join().ok())
}

struct CProduct<'a, T, const SIZE: usize, const OUT_SIZE: usize> {
	src: &'a [T; SIZE],
	idx: [usize; OUT_SIZE],
	end: bool,
}

trait CProductExt<T, const SIZE: usize> {
	fn c_product<const OUT_SIZE: usize>(&self) -> CProduct<T, SIZE, OUT_SIZE>;
}

impl<T: Clone, const SIZE: usize> CProductExt<T, SIZE> for [T; SIZE] {
	fn c_product<const OUT_SIZE: usize>(&self) -> CProduct<T, SIZE, OUT_SIZE> {
		CProduct {
			src: self,
			idx: [0; OUT_SIZE],
			end: SIZE == 0 || OUT_SIZE == 0,
		}
	}
}

impl<'a, T: Clone, const SIZE: usize, const OUT_SIZE: usize> Iterator
	for CProduct<'a, T, SIZE, OUT_SIZE>
{
	type Item = [T; OUT_SIZE];
	fn next(&mut self) -> Option<[T; OUT_SIZE]> {
		if self.end {
			return None;
		}

		let mut res = [const { MaybeUninit::uninit() }; OUT_SIZE];
		let mut i = 0;
		while i < OUT_SIZE {
			res[i] = MaybeUninit::new(self.src[self.idx[i]].clone());
			if self.idx[i] != SIZE - 1 {
				self.idx[i] += 1;
				for i in 0..i {
					self.idx[i] = 0;
				}
				break;
			}

			i += 1;
		}

		if i == OUT_SIZE {
			self.end = true;
			return Some(unsafe { transmute_copy(&ManuallyDrop::new(res)) });
		}

		for i in (i + 1)..OUT_SIZE {
			res[i] = MaybeUninit::new(self.src[self.idx[i]].clone());
		}

		Some(unsafe { transmute_copy(&ManuallyDrop::new(res)) })
	}
}

struct FindOverlapping<'a, T> {
	t: &'a [T],
	pattern: &'a [T],
	idx: usize,
}

impl<'a, T: Eq> Iterator for FindOverlapping<'a, T> {
	type Item = usize;
	fn next(&mut self) -> Option<usize> {
		if self.idx + self.pattern.len().max(1) > self.t.len() {
			return None;
		}

		while !self.t[self.idx..].starts_with(self.pattern) {
			self.idx += 1;
			if self.idx + self.pattern.len().max(1) > self.t.len() {
				return None;
			}
		}

		let v = Some(self.idx);
		self.idx += 1;
		v
	}
}

trait FindOverlappingExt<T> {
	fn find_overlapping<'a>(&'a self, pattern: &'a [T]) -> FindOverlapping<'a, T>;
}

impl<T: Eq> FindOverlappingExt<T> for [T] {
	fn find_overlapping<'a>(&'a self, pattern: &'a [T]) -> FindOverlapping<'a, T> {
		FindOverlapping {
			t: self,
			pattern,
			idx: 0,
		}
	}
}

struct FindNotOverlapping<'a, T> {
	t: &'a [T],
	pattern: &'a [T],
	idx: usize,
}

impl<'a, T: Eq> Iterator for FindNotOverlapping<'a, T> {
	type Item = usize;
	fn next(&mut self) -> Option<usize> {
		if self.idx + self.pattern.len().max(1) > self.t.len() {
			return None;
		}

		while !self.t[self.idx..].starts_with(self.pattern) {
			self.idx += 1;
			if self.idx + self.pattern.len().max(1) > self.t.len() {
				return None;
			}
		}

		let v = Some(self.idx);
		self.idx += self.pattern.len();
		v
	}
}

trait FindNotOverlappingExt<T> {
	fn find_not_overlapping<'a>(&'a self, pattern: &'a [T]) -> FindNotOverlapping<'a, T>;
}

impl<T: Eq> FindNotOverlappingExt<T> for [T] {
	fn find_not_overlapping<'a>(&'a self, pattern: &'a [T]) -> FindNotOverlapping<'a, T> {
		FindNotOverlapping {
			t: self,
			pattern,
			idx: 0,
		}
	}
}
