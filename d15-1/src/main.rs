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
		.cloned()
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
	for m in l2.into_iter() {
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
			l1[y][x + 1] = '.';

			for i in (x + 2)..=w {
				l1[y][i] = 'O';
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

			l1[y][x - 1] = '.';

			for i in w..=(x.max(2) - 2) {
				l1[y][i] = 'O';
			}
			x -= 1;
			continue;
		}
		if m == '^' {
			let Some(w) = (0..y)
				.rev()
				.take_while(|y| l1[*y][x] != '#')
				.filter(|y| l1[*y][x] == '.')
				.next()
			else {
				continue;
			};

			l1[y - 1][x] = '.';

			for i in w..=(y.max(2) - 2) {
				l1[i][x] = 'O';
			}
			y -= 1;
			continue;
		}
		if m == 'v' {
			let Some(w) = ((y + 1)..len_y)
				.take_while(|y| l1[*y][x] != '#')
				.filter(|y| l1[*y][x] == '.')
				.next()
			else {
				continue;
			};

			l1[y + 1][x] = '.';

			for i in (y + 2)..=w {
				l1[i][x] = 'O';
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
			if *d == 'O' {
				sum += i * 100 + j;
			}
		}
	}

	println!("{sum}")
}
