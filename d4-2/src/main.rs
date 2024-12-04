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

	let mut cc = 0;
	for row in lines.windows(3) {
		for ((a, b), c) in row[0]
			.windows(3)
			.zip(row[1].windows(3))
			.zip(row[2].windows(3))
		{
			if b[1] == 'a'
				&& ((a[0] == 'm' && c[2] == 's') || (a[0] == 's' && c[2] == 'm'))
				&& ((a[2] == 'm' && c[0] == 's') || (a[2] == 's' && c[0] == 'm'))
			{
				cc += 1;
			}
		}
	}

	println!("{cc}");
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
