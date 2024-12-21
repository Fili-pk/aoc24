use itertools::Itertools;
use std::{
	collections::{vec_deque, HashMap, HashSet, VecDeque},
	fs::File,
	io::{BufRead, BufReader},
	mem::{swap, transmute_copy, ManuallyDrop, MaybeUninit},
	process::exit,
	thread::scope,
	u64,
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

	let mut cache1 = HashMap::new();
	//let mut cache2 = HashMap::new();
	let mut scrr = 0;
	for line in lines {
		let mut visited = vec![vec![HashSet::<usize>::new(); 3]; 4];
		let mut combs = comb(2, 3, 0, &line, &mut visited, b'\0', 0);

		let scr = combs
			.into_iter()
			.map(|mut x| {
				x.pop();
				x.reverse();
				(find(&x, 0, &mut cache1), x)
			})
			.min_by_key(|x| x.0)
			.unwrap();

		let qqq = line
			.iter()
			.collect::<String>()
			.trim_end_matches(|x: char| !x.is_numeric())
			.parse::<u64>()
			.unwrap();
		scrr += qqq * scr.0;
		// for x in scr.1.iter() {
		// 	print!("{}", *x as char);
		// }
		// println!();

		// print!("{} ", scr.0);
		// let a = find2(&scr.1, 0, &mut cache2);
		// for x in a.1.iter() {
		// 	print!("{}", *x as char);
		// }
		// println!();

		// let mut d = a.1;
		// let mut d2 = vec![];
		// for _ in 0..2 {
		// 	swap(&mut d, &mut d2);
		// 	d = vec![];
		// 	let mut x = 2;
		// 	let mut y = 0;
		// 	for q in &d2 {
		// 		if map2[y][x] == b'\0' {
		// 			break;
		// 		}

		// 		match *q {
		// 			b'<' => x -= 1,
		// 			b'>' => x += 1,
		// 			b'v' => y += 1,
		// 			b'^' => y -= 1,
		// 			b'A' => d.push(map2[y][x]),
		// 			_ => panic!(),
		// 		}
		// 	}

		// 	for x in d.iter() {
		// 		print!("{}", *x as char);
		// 	}

		// 	println!();
		// }

		// let mut x = 2;
		// let mut y = 3;
		// for q in &d {
		// 	if map[y][x] == '\0' {
		// 		panic!(); //<vA<AA>^>AAvA<^A>AvA^A<v<A>^>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
		// 	}
		// 	match *q {
		// 		b'<' => x -= 1,
		// 		b'>' => x += 1,
		// 		b'v' => y += 1,
		// 		b'^' => y -= 1,
		// 		b'A' => print!("{}", map[y][x]),
		// 		_ => panic!(),
		// 	}
		// }
		// println!();
	}

	println!("{scrr}")
}

fn find2(
	path: &[u8],
	depth: u8,
	cache: &mut HashMap<(Vec<u8>, u8), (u64, Vec<u8>)>,
) -> (u64, Vec<u8>) {
	if depth == 25 {
		return (path.len() as u64, path.to_vec());
	}
	let mut sum = 0;
	let mut bbb = vec![];
	let l = path.split(|x| *x == b'A').collect::<Vec<_>>();
	let lll = l.len();

	for p in l.iter().take(lll - 1) {
		let a = p.to_vec();
		if let Some(v) = cache.get(&(a, depth)) {
			sum += v.0;
			bbb.append(&mut v.1.clone());
			continue;
		}

		let mut min = u64::MAX;
		let mut m = vec![];
		let mut visited = vec![vec![HashSet::<usize>::new(); 3]; 2];
		let a = comb2(2, 0, 0, p, b'\0', &mut visited);
		let lenn = a.len();
		for (i, mut x) in a.into_iter().enumerate() {
			x.pop();
			x.reverse();
			x.push(b'A');

			let mut a = find2(&x, depth + 1, cache);
			if a.0 < min {
				min = a.0;
				m = a.1
			}
		}

		sum += min;
		bbb.append(&mut m.clone());

		cache.insert((p.to_vec(), depth), (min, m));
	}

	(sum, bbb)
}

fn find(path: &[u8], depth: u8, cache: &mut HashMap<(Vec<u8>, u8), u64>) -> u64 {
	if depth == 25 {
		return path.len() as u64;
	}
	let mut sum = 0;
	let l = path.split(|x| *x == b'A').collect::<Vec<_>>();
	let lll = l.len();
	for p in l.iter().take(lll - 1) {
		let a = p.to_vec();
		if let Some(v) = cache.get(&(a, depth)) {
			sum += v;
			continue;
		}

		let mut min = u64::MAX;
		let mut visited = vec![vec![HashSet::<usize>::new(); 3]; 2];
		let a = comb2(2, 0, 0, p, b'\0', &mut visited);
		let lenn = a.len();
		for (i, mut x) in a.into_iter().enumerate() {
			x.pop();
			x.reverse();
			x.push(b'A');

			let a = find(&x, depth + 1, cache);
			if a < min {
				min = a;
			}
		}

		sum += min;

		cache.insert((p.to_vec(), depth), min);
	}

	sum
}

fn comb2(
	x: i32,
	y: i32,
	mut idx: usize,
	pat: &[u8],
	dir: u8,
	vis: &mut Vec<Vec<HashSet<usize>>>,
) -> Vec<Vec<u8>> {
	let Some(c) = map2.geti2(y, x) else {
		return vec![];
	};

	if *c == b'\0' {
		return vec![];
	}

	if vis[y as usize][x as usize].contains(&idx) {
		return vec![];
	}

	if idx >= pat.len() {
		if x == 2 && y == 0 {
			return vec![vec![dir]];
		}

		let mut a = vec![];
		vis[y as usize][x as usize].insert(idx);
		a.append(&mut comb2(x, y - 1, idx, pat, b'^', vis));
		a.append(&mut comb2(x, y + 1, idx, pat, b'v', vis));
		a.append(&mut comb2(x - 1, y, idx, pat, b'<', vis));
		a.append(&mut comb2(x + 1, y, idx, pat, b'>', vis));
		for a in &mut a {
			a.push(dir);
		}
		vis[y as usize][x as usize].remove(&idx);

		return a;
	}

	let p = pat[idx];
	if p == *c {
		let mut a = vec![];
		let mut d = 0;
		while let Some(p) = pat.get(idx) {
			if c != p {
				break;
			}

			idx += 1;
			d += 1;
		}
		a.append(&mut comb2(x + 1, y, idx, pat, b'>', vis));
		a.append(&mut comb2(x - 1, y, idx, pat, b'<', vis));
		a.append(&mut comb2(x, y + 1, idx, pat, b'v', vis));
		a.append(&mut comb2(x, y - 1, idx, pat, b'^', vis));
		for a in &mut a {
			for _ in 0..d {
				a.push(b'A');
			}
			a.push(dir);
		}

		return a;
	}

	vis[y as usize][x as usize].insert(idx);
	let mut a = vec![];
	a.append(&mut comb2(x + 1, y, idx, pat, b'>', vis));
	a.append(&mut comb2(x - 1, y, idx, pat, b'<', vis));
	a.append(&mut comb2(x, y + 1, idx, pat, b'v', vis));
	a.append(&mut comb2(x, y - 1, idx, pat, b'^', vis));
	vis[y as usize][x as usize].remove(&idx);
	for a in &mut a {
		a.push(dir);
	}

	a
}

fn comb(
	x: i32,
	y: i32,
	idx: usize,
	pat: &Vec<char>,
	vis: &mut Vec<Vec<HashSet<usize>>>,
	dir: u8,
	depth: u8,
) -> Vec<Vec<u8>> {
	if depth == 20 {
		return vec![];
	}

	if idx >= pat.len() {
		return vec![vec![]];
	}

	let p = pat[idx];
	let Some(v) = map.geti2(y, x) else {
		return vec![];
	};

	if *v == '\0' {
		return vec![];
	}

	if vis[y as usize][x as usize].contains(&idx) {
		return vec![];
	}

	if p == *v {
		let mut a = vec![];
		a.append(&mut comb(x + 1, y, idx + 1, pat, vis, b'>', depth + 1));
		a.append(&mut comb(x - 1, y, idx + 1, pat, vis, b'<', depth + 1));
		a.append(&mut comb(x, y + 1, idx + 1, pat, vis, b'v', depth + 1));
		a.append(&mut comb(x, y - 1, idx + 1, pat, vis, b'^', depth + 1));
		for a in &mut a {
			a.push(b'A');
			a.push(dir);
		}

		return a;
	}

	vis[y as usize][x as usize].insert(idx);
	let mut a = vec![];
	a.append(&mut comb(x + 1, y, idx, pat, vis, b'>', depth + 1));
	a.append(&mut comb(x - 1, y, idx, pat, vis, b'<', depth + 1));
	a.append(&mut comb(x, y + 1, idx, pat, vis, b'v', depth + 1));
	a.append(&mut comb(x, y - 1, idx, pat, vis, b'^', depth + 1));
	for a in &mut a {
		a.push(dir);
	}
	vis[y as usize][x as usize].remove(&idx);
	a
}

const map: [[char; 3]; 4] = [
	['7', '8', '9'],
	['4', '5', '6'],
	['1', '2', '3'],
	['\0', '0', 'A'],
];
const map2: [[u8; 3]; 2] = [[b'\0', b'^', b'A'], [b'<', b'v', b'>']];

trait GetI2Ext<T> {
	fn geti2(&self, y: i32, x: i32) -> Option<&T>;
	fn geti2_mut(&mut self, y: i32, x: i32) -> Option<&mut T>;
}

impl<T, const S: usize> GetI2Ext<T> for [[T; S]] {
	fn geti2(&self, y: i32, x: i32) -> Option<&T> {
		self.geti(y).and_then(|d| d.geti(x))
	}

	fn geti2_mut(&mut self, y: i32, x: i32) -> Option<&mut T> {
		self.geti_mut(y).and_then(|d| d.geti_mut(x))
	}
}
