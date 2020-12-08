#![deny(rust_2018_idioms, future_incompatible)]
#![feature(type_ascription)]

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let list = include_str!("input.txt")
		.lines()
		.map(str::parse)
		.collect(): Result<Vec<i32>, _>;

	let list = list?;

	println!("part_1: {:?}", part_1(&list));
	println!("part_2: {:?}", part_2(&list));


	Ok(())
}

fn part_1(list: &[i32]) -> Option<i32> {
	for (i, &a) in list.iter().enumerate() {
		for &b in list[i+1..].iter() {
			if a + b == 2020 {
				return Some(a * b);
			}
		}
	}

	None
}

fn part_2(list: &[i32]) -> Option<i32> {
	for (i, &a) in list.iter().enumerate() {
		for (j, &b) in list[i+1..].iter().enumerate() {
			for &c in list[i+j+1..].iter() {
				if a + b + c == 2020 {
					return Some(a * b * c);
				}
			}
		}
	}

	None
}