use itertools::Itertools;
use std::{
	fs::File,
	io::{BufRead, BufReader},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum P {
	Obs,
	None,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum D {
	Up,
	Down,
	Left,
	Right,
}

impl D {
	fn right(self) -> Self {
		match self {
			Self::Up => Self::Right,
			Self::Right => Self::Down,
			Self::Down => Self::Left,
			Self::Left => Self::Up,
		}
	}
}

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let mut pos_x = 0;
	let mut pos_y = 0;
	let mut d = D::Up;
	let lines: Vec<Vec<_>> = file
		.lines()
		.enumerate()
		.map(|(i, x)| {
			let x = x.unwrap();
			x.chars()
				.enumerate()
				.map(|(j, x)| match x {
					'.' => P::None,
					'#' => P::Obs,
					'^' => {
						d = D::Up;
						pos_x = j;
						pos_y = i;
						P::None
					}
					'v' => {
						d = D::Down;
						pos_x = j;
						pos_y = i;
						P::None
					}
					'<' => {
						d = D::Left;
						pos_x = j;
						pos_y = i;
						P::None
					}
					'>' => {
						d = D::Right;
						pos_x = j;
						pos_y = i;
						P::None
					}
					_ => panic!(),
				})
				.collect()
		})
		.collect();

	let mut visited = vec![vec![false; lines[0].len()]; lines.len()];
	let mut scr = 0;
	loop {
		if !visited[pos_y][pos_x] {
			scr += 1;
			visited[pos_y][pos_x] = true;
		}

		match d {
			D::Up => {
				if lines[pos_y - 1][pos_x] == P::None {
					pos_y -= 1
				} else {
					d = d.right()
				}
			}

			D::Left => {
				if lines[pos_y][pos_x - 1] == P::None {
					pos_x -= 1
				} else {
					d = d.right()
				}
			}

			D::Down => {
				if lines[pos_y + 1][pos_x] == P::None {
					pos_y += 1
				} else {
					d = d.right()
				}
			}

			D::Right => {
				if lines[pos_y][pos_x + 1] == P::None {
					pos_x += 1
				} else {
					d = d.right()
				}
			}
		}

		// for x in &visited {
		// 	for x in x {
		// 		print!("{}", *x as u8)
		// 	}

		// 	println!()
		// }
		println!("{}", scr + 1) //if panic then outside
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
