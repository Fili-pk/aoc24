use itertools::Itertools;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
};

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let lines: Vec<_> = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
		})
		.collect();
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
