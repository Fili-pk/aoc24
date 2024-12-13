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

	let mut w: Vec<Vec<(f64, f64)>> = lines
		.chunks_exact(4)
		.map(|x| {
			x.iter()
				.take(3)
				.map(|x| x.split_once(": ").unwrap().1.split_once(", ").unwrap())
				.map(|x| {
					(
						x.0.trim_start_matches("X+")
							.trim_start_matches("X=")
							.parse::<i32>()
							.unwrap() as f64,
						x.1.trim_start_matches("Y+")
							.trim_start_matches("Y=")
							.parse::<i32>()
							.unwrap() as f64,
					)
				})
				.collect()
		})
		.collect();

	for w in w.iter_mut() {
		w[2].0 += 10000000000000.0;
		w[2].1 += 10000000000000.0;
	}

	let mut summ: u64 = 0;
	for mut d in w {
		let w_x = d[0].0 / d[0].1;
		d.iter_mut().for_each(|x| x.0 -= x.1 * w_x);
		let w_y = d[1].1 / d[1].0;
		d.iter_mut().for_each(|x| x.1 -= x.0 * w_y);
		let s_y = d[0].1;
		d.iter_mut().for_each(|x| x.1 /= s_y);
		let s_x = d[1].0;
		d.iter_mut().for_each(|x| x.0 /= s_x);

		if (d[2].1 - d[2].1.round()).abs() < 0.05 && (d[2].0 - d[2].0.round()).abs() < 0.05 {
			println!("{} {}", d[2].0, d[2].1);
			summ += d[2].0.round() as u64 + d[2].1.round() as u64 * 3;
		}
	}

	println!("{summ}")
}
