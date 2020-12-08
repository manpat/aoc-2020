#![deny(rust_2018_idioms, future_incompatible)]
#![feature(type_ascription, str_split_once)]


fn main() {
	let data = include_str!("input.txt");
	let map_width = data.chars().position(|c| c == '\n')
		.expect("couldn't find newline");

	let map_data = data.chars()
		.filter_map(|ch| match ch {
			'.' => Some(Tile::Open),
			'#' => Some(Tile::Tree),
			_ => None
		})
		.collect(): Vec<_>;

	let part_1 = trees_on_diagonal(&map_data, map_width, 3, 1);
	dbg!(part_1);

	let slopes = [
		(1, 1),
		(3, 1),
		(5, 1),
		(7, 1),
		(1, 2),
	];

	let part_2 = slopes.iter()
		.map(|&(sx, sy)| trees_on_diagonal(&map_data, map_width, sx, sy))
		.product(): usize;

	dbg!(part_2);
}


#[derive(Debug, Copy, Clone)]
enum Tile {
	Open,
	Tree,
}


fn trees_on_diagonal(data: &[Tile], map_width: usize,
	slope_x: usize, slope_y: usize) -> usize
{
	let map_height = data.len() / map_width;
	let samples = map_height / slope_y;

	(0..samples)
		.map(|row| {
			let column = row * slope_x % map_width;
			let index = row * map_width * slope_y + column;
			match data[index] {
				Tile::Tree => 1,
				Tile::Open => 0,
			}
		})
		.sum()
}