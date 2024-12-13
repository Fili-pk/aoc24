use itertools::Itertools;
use rayon::prelude::*;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
	thread::scope,
};
use utils::*;

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();

	let w: Vec<Vec<(i32, i32)>> = lines
		.chunks_exact(4)
		.map(|x| {
			x.iter()
				.take(3)
				.map(|x| x.split_once(": ").unwrap().1.split_once(", ").unwrap())
				.map(|x| {
					(
						x.0.trim_start_matches("X+")
							.trim_start_matches("X=")
							.parse()
							.unwrap(),
						x.1.trim_start_matches("Y+")
							.trim_start_matches("Y=")
							.parse()
							.unwrap(),
					)
				})
				.collect()
		})
		.collect();

	let mut summ: i32 = (w.par_iter())
		.map(|d| {
			let q = (0..10000)
				.cartesian_product(0..10000)
				.filter(|(i, j)| {
					let j = *j;
					let i = *i;
					d[0].0 * i + d[1].0 * j == d[2].0 && d[0].1 * i + d[1].1 * j == d[2].1
				})
				.map(|(i, j)| i * 3 + j)
				.min();

			q.unwrap_or(0)
		})
		.sum();

	println!("{summ}")
}
