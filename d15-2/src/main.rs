use itertools::Itertools;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
	thread::scope,
};
use utils::*;

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let lines: Vec<_> = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
			x.chars().collect::<Vec<_>>()
		})
		.collect();

	let mut l1 = lines
		.iter()
		.take_while(|x| !x.is_empty())
		.map(|x| {
			x.iter()
				.flat_map(|x| {
					match *x {
						'#' => ['#', '#'],
						'.' => ['.', '.'],
						'O' => ['[', ']'],
						'@' => ['@', '.'],
						_ => panic!(),
					}
					.into_iter()
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	let l2 = lines
		.iter()
		.filter(|x| !x.contains(&'#') && !x.is_empty())
		.map(|x| x.iter())
		.flatten()
		.copied()
		.collect::<Vec<_>>();

	let mut x = 1;
	let mut y = 1;
	for (i, d) in l1.iter_mut().enumerate() {
		for (j, d) in d.iter_mut().enumerate() {
			if *d == '@' {
				x = j;
				y = i;
				*d = '.';
			}
		}
	}

	let len_y = l1.len();
	let len_x = l1[0].len();
	// println!("{:?}", l2);
	for (qqq, m) in l2.into_iter().enumerate() {
		// println!("{qqq}");
		// for (i, d) in l1.iter().enumerate() {
		// 	for (j, d) in d.iter().enumerate() {
		// 		if i == y && j == x {
		// 			print!("@");
		// 			continue;
		// 		}
		// 		print!("{d}")
		// 	}
		// 	println!()
		// }
		// println!();
		if m == '>' {
			let Some(w) = ((x + 1)..len_x)
				.take_while(|x| l1[y][*x] != '#')
				.filter(|x| l1[y][*x] == '.')
				.next()
			else {
				continue;
			};

			for i in ((x + 2)..=w).rev() {
				l1[y].swap(i - 1, i);
			}
			x += 1;
			continue;
		}
		if m == '<' {
			let Some(w) = (0..x)
				.rev()
				.take_while(|x| l1[y][*x] != '#')
				.filter(|x| l1[y][*x] == '.')
				.next()
			else {
				continue;
			};

			for i in w..=(x - 2) {
				l1[y].swap(i, i + 1);
			}
			x -= 1;
			continue;
		}
		if m == '^' {
			if l1[y - 1][x] == '#' {
				continue;
			}

			if l1[y - 1][x] == '.' {
				y -= 1;
				continue;
			}

			let c = l1.clone();
			if l1[y - 1][x] == '[' {
				up(&mut l1, x, y - 1);
			} else {
				up(&mut l1, x - 1, y - 1);
			}

			if l1[y - 1][x] != '.' {
				l1 = c;
				continue;
			}

			y -= 1;
			continue;
		}
		if m == 'v' {
			if l1[y + 1][x] == '#' {
				continue;
			}

			if l1[y + 1][x] == '.' {
				y += 1;
				continue;
			}

			let c = l1.clone();
			if l1[y + 1][x] == '[' {
				down(&mut l1, x, y + 1);
			} else {
				down(&mut l1, x - 1, y + 1);
			}

			if l1[y + 1][x] != '.' {
				l1 = c;
				continue;
			}
			y += 1;
			continue;
		}
	}

	for x in l1.iter() {
		for x in x {
			print!("{x}")
		}
		println!()
	}

	let mut sum = 0;
	for (i, d) in l1.iter().enumerate() {
		for (j, d) in d.iter().enumerate() {
			if *d == '[' {
				sum += i * 100 + j;
			}
		}
	}

	println!("{sum}")
}

fn up(l1: &mut Vec<Vec<char>>, x: usize, y: usize) {
	if l1[y - 1][x] == '#' || l1[y - 1][x + 1] == '#' {
		return;
	}

	if l1[y - 1][x] == '[' {
		up(l1, x, y - 1);
	}

	if l1[y - 1][x] == ']' {
		up(l1, x - 1, y - 1);
	}

	if l1[y - 1][x + 1] == '[' {
		up(l1, x + 1, y - 1);
	}

	if l1[y - 1][x] == '.' && l1[y - 1][x + 1] == '.' {
		l1[y - 1][x] = '[';
		l1[y - 1][x + 1] = ']';
		l1[y][x] = '.';
		l1[y][x + 1] = '.';
	}
}

fn down(l1: &mut Vec<Vec<char>>, x: usize, y: usize) {
	if l1[y + 1][x] == '#' || l1[y + 1][x + 1] == '#' {
		return;
	}

	if l1[y + 1][x] == '[' {
		down(l1, x, y + 1);
	}

	if l1[y + 1][x] == ']' {
		down(l1, x - 1, y + 1);
	}

	if l1[y + 1][x + 1] == '[' {
		down(l1, x + 1, y + 1);
	}

	if l1[y + 1][x] == '.' && l1[y + 1][x + 1] == '.' {
		l1[y + 1][x] = '[';
		l1[y + 1][x + 1] = ']';
		l1[y][x] = '.';
		l1[y][x + 1] = '.';
	}
}
