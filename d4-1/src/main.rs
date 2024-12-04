use itertools::Itertools;
use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let mut lines: Vec<Vec<_>> = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
			x.chars().map(|x| x.to_ascii_lowercase()).collect()
		})
		.collect();

	let mut c = 0;
	for l in lines.iter() {
		for i in 0..l.len() {
			c += l[i..].starts_with(&['x', 'm', 'a', 's']) as u8 as i32;
			c += l[i..].starts_with(&['s', 'a', 'm', 'x']) as u8 as i32;
		}
	}

	let t = transpose(lines.clone());
	for l in t.iter() {
		for i in 0..l.len() {
			c += l[i..].starts_with(&['x', 'm', 'a', 's']) as u8 as i32;
			c += l[i..].starts_with(&['s', 'a', 'm', 'x']) as u8 as i32;
		}
	}

	let len = lines.len();
	let mut l2 = lines.clone();
	for (i, x) in l2.iter_mut().enumerate() {
		for j in 0..i {
			x.insert(0, '\0');
		}

		for j in 0..len - i {
			x.push('\0');
		}
	}

	let t = transpose(l2.clone());
	for l in t.iter() {
		for i in 0..l.len() {
			c += l[i..].starts_with(&['x', 'm', 'a', 's']) as u8 as i32;
			c += l[i..].starts_with(&['s', 'a', 'm', 'x']) as u8 as i32;
		}
	}

	let len = lines.len();
	let mut l2 = lines.clone();
	for (i, x) in l2.iter_mut().enumerate() {
		for j in 0..len - i {
			x.insert(0, '\0');
		}

		for j in 0..i {
			x.push('\0');
		}
	}

	let t = transpose(l2.clone());
	for l in t.iter() {
		for i in 0..l.len() {
			c += l[i..].starts_with(&['x', 'm', 'a', 's']) as u8 as i32;
			c += l[i..].starts_with(&['s', 'a', 'm', 'x']) as u8 as i32;
		}
	}

	println!("{c}");
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
	assert!(!v.is_empty());
	let len = v[0].len();
	let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
	(0..len)
		.map(|_| {
			iters
				.iter_mut()
				.map(|n| n.next().unwrap())
				.collect::<Vec<T>>()
		})
		.collect()
}
