use itertools::Itertools;
use std::{
	collections::HashMap,
	fs::File,
	io::{BufRead, BufReader},
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
	thread::scope,
	u128,
};
use utils::*;

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let mut lines: Vec<_> = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
			x.split(" ")
				.map(|x| x.parse().unwrap())
				.collect::<Vec<u128>>()
		})
		.next()
		.unwrap();

	const STEPS: usize = 75;
	let l = lines.len();
	let mut cache = HashMap::new();
	let mut v = vec![vec![]; STEPS + 1];
	v[0] = lines;

	let mut sum = vec![0u128; STEPS];
	'a: while !v[0].is_empty() {
		for i in 0..STEPS {
			if !v[i + 1].is_empty() {
				continue;
			}

			let w: u128 = *v[i].last().unwrap();
			if let Some(a) = cache.get(&(w, i)) {
				v[i].pop();
				sum[i] += a;
				for i in (1..=i).rev() {
					if !v[i].is_empty() {
						break;
					}

					sum[i - 1] += sum[i];
					cache.insert((v[i - 1].pop().unwrap(), i - 1), sum[i]);
					sum[i] = 0;
				}
				continue 'a;
			}

			if w == 0 {
				v[i + 1].push(1);
				continue;
			}

			let d = (w as f32).log10().floor() as u32 + 1;
			if d % 2 == 0 {
				let q = 10u128.pow(d / 2);
				v[i + 1].push(w % q);
				v[i + 1].push(w / q);
				continue;
			}

			v[i + 1].push(w * 2024);
		}

		let len = v[STEPS].len() as u128;
		v[STEPS] = vec![];
		sum[STEPS - 1] += len;
		cache.insert((v[STEPS - 1].pop().unwrap(), STEPS - 1), len);
		for i in (1..STEPS).rev() {
			if !v[i].is_empty() {
				break;
			}

			sum[i - 1] += sum[i];
			cache.insert((v[i - 1].pop().unwrap(), i - 1), sum[i]);
			sum[i] = 0;
		}
	}

	println!("{:?}", sum);
}
