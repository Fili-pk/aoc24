use itertools::Itertools;
use std::{
	collections::{HashMap, HashSet},
	fs::File,
	i32,
	io::{BufRead, BufReader},
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
	thread::scope,
};
use utils::*;

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let w: Vec<_> = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
			x.chars().collect::<Vec<_>>()
		})
		.collect();

	let mut test = HashMap::new();
	let mut l = vec![vec![0i32; w[0].len()]; w.len()];
	for i in 0..l.len() {
		for j in 0..l[0].len() {
			test.insert(i as i32 * 10000 + j as i32 + 1, w[i][j]);
			rec(
				&mut l,
				&w,
				i as i32,
				j as i32,
				w[i][j],
				i as i32 * 10000 + j as i32 + 1,
			);
		}
	}

	let mut map1 = HashMap::new();
	let mut map2 = HashMap::new();
	for i in 0..l.len() as i32 {
		for j in 0..l[0].len() as i32 {
			let x = l.geti(i).unwrap().geti(j).unwrap();
			map1.entry(*x).and_modify(|x| *x += 1).or_insert(1);
		}
	}

	for i in (-1)..(l.len() as i32 + 1) {
		let mut last_a = -3;
		let mut last_b = -3;
		for j in (-1)..(l[0].len() as i32 + 1) {
			let x = *l.geti(i).and_then(|x| x.geti(j)).unwrap_or(&i32::MAX);
			if x == last_a {
				last_a = -3
			}

			if x == last_b {
				last_b = -3
			}

			if l.geti(i - 1)
				.and_then(|x| x.geti(j))
				.is_some_and(|w| *w == x)
			{
				last_a = -3
			}

			if l.geti(i + 1)
				.and_then(|x| x.geti(j))
				.is_some_and(|w| *w == x)
			{
				last_b = -3
			}
			if let Some(x) = l
				.geti(i - 1)
				.and_then(|x| x.geti(j))
				.and_then(|q| q.iff(|w| **w != x))
				.and_then(|q| q.iff(|w| **w != last_a))
			{
				println!("{i}, {j}, {}", test[&x]);
				map2.entry(x).and_modify(|x| *x += 1).or_insert(1);
				last_a = *x;
			}

			if let Some(x) = l
				.geti(i + 1)
				.and_then(|x| x.geti(j))
				.and_then(|q| q.iff(|w| **w != x))
				.and_then(|q| q.iff(|w| **w != last_b))
			{
				println!("{i}, {j}, {}", test[&x]);
				map2.entry(x).and_modify(|x| *x += 1).or_insert(1);
				last_b = *x;
			}
		}
	}

	for j in (-1)..(l[0].len() as i32 + 1) {
		let mut last_a = -3;
		let mut last_b = -3;
		for i in (-1)..(l.len() as i32 + 1) {
			if j == 6 && i == 4 {
				println!()
			}
			let x = *l.geti(i).and_then(|x| x.geti(j)).unwrap_or(&i32::MAX);
			if l.geti(i)
				.and_then(|x| x.geti(j - 1))
				.is_some_and(|w| *w == x)
			{
				last_a = -3
			}

			if l.geti(i)
				.and_then(|x| x.geti(j + 1))
				.is_some_and(|w| *w == x)
			{
				last_b = -3
			}

			if x == last_a {
				last_a = -3
			}

			if x == last_b {
				last_b = -3
			}

			if let Some(x) = l
				.geti(i)
				.and_then(|x| x.geti(j - 1))
				.and_then(|q| q.iff(|w| **w != x))
				.and_then(|q| q.iff(|w| **w != last_a))
			{
				println!("{i}, {j}, {}", test[&x]);
				map2.entry(x).and_modify(|x| *x += 1).or_insert(1);
				last_a = *x;
			}
			if let Some(x) = l
				.geti(i)
				.and_then(|x| x.geti(j + 1))
				.and_then(|q| q.iff(|w| **w != x))
				.and_then(|q| q.iff(|w| **w != last_b))
			{
				println!("{i}, {j}, {}", test[&x]);
				map2.entry(x).and_modify(|x| *x += 1).or_insert(1);
				last_b = *x;
			}
		}
	}

	let mut sum = 0;
	for x in map1 {
		println!("{} {} {}", test[&x.0], map2[&x.0], x.1);
		sum += x.1 * map2[&x.0];
	}

	println!("{sum}")
}

fn rec(l: &mut Vec<Vec<i32>>, w: &Vec<Vec<char>>, i: i32, j: i32, c: char, n: i32) {
	if !w.geti(i).and_then(|x| x.geti(j)).is_some_and(|x| *x == c) {
		return;
	}

	if l[i as usize][j as usize] != 0 {
		return;
	}

	l[i as usize][j as usize] = n;

	rec(l, w, i + 1, j, c, n);
	rec(l, w, i, j + 1, c, n);
	rec(l, w, i - 1, j, c, n);
	rec(l, w, i, j - 1, c, n);
}
