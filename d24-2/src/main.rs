use itertools::Itertools;
use std::{
	collections::HashMap,
	fmt::format,
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
			x
		})
		.collect();

	let mut d = vec![];
	for x in lines.iter().filter(|x| x.contains("->")) {
		let x = x.split_once(" ").unwrap();
		let x2 = x.1.split_once(" ").unwrap();
		let x3 = x2.1.split_once(" -> ").unwrap();
		d.push((
			x.0.to_owned(),
			x2.0.to_owned(),
			x3.0.to_owned(),
			x3.1.to_owned(),
		));
	}

	const SIZE: usize = 45;
	for x in 0..SIZE {
		let mut h = HashMap::new();
		let mut d = d.clone();
		for x in 0..SIZE {
			h.insert(format!("x{x:0>2}"), false);
			h.insert(format!("y{x:0>2}"), true);
		}

		h.remove(&format!("x{x:0>2}"));
		h.insert(format!("x{x:0>2}"), true);

		while !d.is_empty() {
			let mut i = 0;
			while i < d.len() {
				let x = &d[i];
				let Some(_) = h.get(&x.0) else {
					i += 1;
					continue;
				};
				let Some(_) = h.get(&x.2) else {
					i += 1;
					continue;
				};

				let x = d.swap_remove(i);
				let x1 = *h.get(&x.0).unwrap();
				let x2 = *h.get(&x.2).unwrap();
				let v = match x.1.as_str() {
					"AND" => x1 && x2,
					"XOR" => x1 != x2,
					"OR" => x1 || x2,
					x => panic!("{}", x),
				};

				h.remove(&x.3);
				h.insert(x.3.clone(), v);
			}
		}

		for x in 0..=SIZE {
			print!("{}", *h.get(&format!("z{x:0>2}")).unwrap() as u8)
		}

		println!();
	}

	// DEBUGED IT AND FOUND ANSWER
	// YEARS OF REDSTONE PAID OFF
	//jbc OR mnv -> z32
	//kcv XOR pqv -> gfm

	//y08 AND x08 -> z08
	//dnc XOR rtp -> cdj

	//btj AND tmm -> z16
	//btj XOR tmm -> mrb

	//x38 AND y38 -> qjd
	//x38 XOR y38 -> dhm
}
