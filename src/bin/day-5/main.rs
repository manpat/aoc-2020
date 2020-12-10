#![deny(rust_2018_idioms, future_incompatible)]
#![feature(type_ascription, str_split_once)]

fn main() {
	let mut passes = include_str!("input.txt")
		.lines()
		.map(parse_boarding_pass)
		.collect(): Vec<_>;

	passes.sort();

	let part_1 = passes.iter().max();
	dbg!(part_1);

	let part_2 = find_missing_seat(&passes);
	dbg!(part_2);
}


type SeatId = usize;

fn parse_boarding_pass(input: &str) -> SeatId {
	input.chars()
		.map(|c| match c {
			'F'|'L' => 0,
			'B'|'R' => 1,
			_ => panic!(),
		})
		.fold(0, |a, d| (a << 1) + d)
}

fn find_missing_seat(taken_seats: &[SeatId]) -> SeatId {
	let prev_seat_id = taken_seats.windows(2)
		.find(|&window| match window {
			&[a, b] => a+1 != b,
			_ => false
		})
		.expect("couldn't find seat");

	prev_seat_id[0] + 1
}