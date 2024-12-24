use itertools::Itertools;
use std::{
	collections::{vec_deque, HashMap, HashSet, VecDeque},
	fs::File,
	io::{BufRead, BufReader},
	mem::{swap, transmute_copy, ManuallyDrop, MaybeUninit},
	thread::scope,
};
use utils::*;

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let mut h = HashMap::new();
	let lines: Vec<_> = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
			x.split_once("-")
				.map(|x| (x.0.to_owned(), x.1.to_owned()))
				.unwrap()
		})
		.collect();

	for a in lines {
		println!("{a:?}");
		h.entry(a.0.clone())
			.and_modify(|x: &mut Vec<String>| x.push(a.1.clone()))
			.or_insert(vec![a.1.clone()]);

		h.entry(a.1)
			.and_modify(|x: &mut Vec<String>| x.push(a.0.clone()))
			.or_insert(vec![a.0]);
	}

	let mut max = vec![];
	for x in h.iter() {
		let lenn = x.1.len();
		'w: for mut x1 in [false, true].c_product::<14>() {
			if x1[lenn] {
				break;
			}

			let mut v = x1
				.iter()
				.zip(&h[x.0])
				.filter(|x| *x.0)
				.map(|x| x.1)
				.collect::<Vec<_>>();

			for x in &v {
				for x2 in &v {
					if !h[*x2].contains(*x) && x != x2 {
						continue 'w;
					}
				}
			}

			v.push(x.0);

			if max.len() < v.len() {
				max = v;
			}
			//println!("{x:?}")
		}
	}

	max.sort();
	for x in max {
		print!("{x},")
	}
	println!()
}

// fn ddd(h: &HashMap<String, Vec<String>>, depth: usize, k: &String) -> Vec<Vec<String>> {
// 	let Some(kk) = h.get(k) else {
// 		return vec![];
// 	};

// 	if depth == 2 {
// 		return kk.iter().map(|x| vec![x.clone()]).collect();
// 	}

// 	let mut w = vec![];
// 	for x in kk {
// 		if x == k {
// 			continue;
// 		}
// 		w.append(&mut ddd(h, depth + 1, x));
// 	}

// 	for w in &mut w {
// 		w.push(k.clone());
// 	}

// 	w
// }
