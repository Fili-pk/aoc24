use image::{Pixel, RgbImage};
use itertools::Itertools;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
	thread::scope,
};
use utils::*;

struct R {
	x: i32,
	y: i32,
	v_x: i32,
	v_y: i32,
}

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let mut lines: Vec<_> = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
			let (a, b) = x.split_once(" ").unwrap();
			let (a1, a2) = a.split_once(",").unwrap();
			let (b1, b2) = b.split_once(",").unwrap();
			R {
				x: a1.trim_start_matches("p=").parse().unwrap(),
				y: a2.parse().unwrap(),
				v_x: b1.trim_start_matches("v=").parse().unwrap(),
				v_y: b2.parse().unwrap(),
			}
		})
		.collect();

	for i in 0..100000 {
		let mut img = RgbImage::new(101, 103);

		if lines
			.iter()
			.filter(|x| (37..67).contains(&x.x) && (39..70).contains(&x.y))
			.count() < 64
		{
			for x in lines.iter_mut() {
				x.x += x.v_x + 101 * 10000;
				x.y += x.v_y + 103 * 10000;
				x.x %= 101;
				x.y %= 103;
			}
			continue;
		}

		for x in lines.iter() {
			img.get_pixel_mut(x.x as u32, x.y as u32).0 = [255, 255, 255];
		}

		img.save(format!("./sus/{i}.png"));

		for x in lines.iter() {
			img.get_pixel_mut(x.x as u32, x.y as u32).0 = [0, 0, 0];
		}

		for x in lines.iter_mut() {
			x.x += x.v_x + 101 * 10000;
			x.y += x.v_y + 103 * 10000;
			x.x %= 101;
			x.y %= 103;
		}
	}
}
