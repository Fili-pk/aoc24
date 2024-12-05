use itertools::Itertools;
use std::{
	collections::HashMap,
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let l: Vec<_> = file.lines().map(|x| x.unwrap()).collect();

	let mut l1: HashMap<i32, Vec<i32>> = HashMap::new();
	l.iter().take_while(|x| !x.is_empty()).for_each(|x| {
		let a = x.split_once('|').unwrap();
		l1.entry(a.0.parse().unwrap())
			.and_modify(|x| x.push(a.1.parse().unwrap()))
			.or_insert(vec![a.1.parse().unwrap()]);
	});

	let l2: Vec<Vec<i32>> = l
		.iter()
		.filter(|x| !x.contains('|') && !x.is_empty())
		.map(|x| x.split(',').map(|x| x.parse().unwrap()).collect())
		.collect();

	let mut lig = vec![];
	'w: for a in l2 {
		for (i, b) in a.iter().enumerate() {
			for c in l1.get(b).unwrap_or(&vec![]).iter() {
				for q in a.iter().take(i) {
					if q == c {
						lig.push(a.clone());
						continue 'w;
					}
				}
			}
		}
	}

	for a in lig.iter_mut() {
		'w: loop {
			for (i, b) in a.iter().enumerate() {
				for c in l1.get(b).unwrap_or(&vec![]).iter() {
					for (j, q) in a.iter().take(i).enumerate() {
						if q == c {
							a.swap(i, j);
							continue 'w;
						}
					}
				}
			}
			break;
		}
	}

	let s: i32 = lig.iter().map(|a| a[a.len() / 2]).sum();

	println!("{s}")
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
