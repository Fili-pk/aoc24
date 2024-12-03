use itertools::Itertools;
use std::{
	fs::File,
	io::{BufRead, BufReader, Read},
};

fn main() {
	let mut file = BufReader::new(File::open("input").unwrap());
	let mut f = String::new();
	file.read_to_string(&mut f).unwrap();
	let f: Vec<_> = f.chars().collect();
	let mut l = 0;

	for i in (0..f.len() - 5).filter(|i| f[*i..].starts_with(&['m', 'u', 'l', '('])) {
		let j = (i..f.len()).take_while(|j| f[*j] != ')').last();
		let Some(j) = j else {
			break;
		};

		let s: String = f[i + 4..=j].iter().collect();
		let Some((Ok(a), Ok(b))): Option<(Result<i32, _>, Result<i32, _>)> =
			s.split_once(',').map(|x| (x.0.parse(), x.1.parse()))
		else {
			continue;
		};

		l += a * b;
	}

	println!("{l}")
}
