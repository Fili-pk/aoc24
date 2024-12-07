use itertools::Itertools;
use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let l: Vec<Vec<i64>> = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
			println!("{x}");
			x.split(' ')
				.map(|x| x.trim_end_matches(':').parse().unwrap())
				.collect()
		})
		.collect();

	let mut scr = 0;
	for l in l {
		let len = l.len();

		for y in dd(len - 2) {
			let mut s = l[1];
			for x in l[2..].iter().zip(&y) {
				if *x.1 == 0 {
					let a = x.0.overflowing_mul(s);
					if a.1 {
						continue;
					}
					s = a.0;
				} else if *x.1 == 1 {
					let a = x.0.overflowing_add(s);
					if a.1 {
						continue;
					}
					s = a.0;
				} else {
					let w = (*x.0 as f64).log10().floor() as u32 + 1;
					if w >= 18 {
						continue;
					}
					let a = 10_i64.pow(w).overflowing_mul(s);
					let b = a.0.overflowing_add(*x.0);
					if a.1 || b.1 {
						continue;
					}
					s = b.0;
				}
			}

			if s == l[0] {
				println!("{:?}\n{:?}", y, l);
				scr += l[0];
				break;
			}
		}
	}
	println!("{scr}")
}

fn dd(l: usize) -> Vec<Vec<u8>> {
	let mut c: Vec<Vec<u8>> = vec![vec![]];

	for _ in 0..l {
		for i in 0..c.len() {
			let mut w = c[i].clone();
			w.push(1);
			c.push(w);
			let mut w = c[i].clone();
			w.push(2);
			c.push(w);
			c[i].push(0);
		}
	}

	c
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
