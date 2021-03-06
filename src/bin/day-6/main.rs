#![deny(rust_2018_idioms, future_incompatible)]
#![feature(type_ascription, str_split_once)]

fn main() {
	let groups = include_str!("input.txt")
		.split("\n\n");

	let part_1 = groups.clone()
		.map(questions_answered_by_group)
		.sum(): usize;

	dbg!(part_1);

	let part_2 = groups
		.map(common_questions_answered_by_group)
		.sum(): usize;

	dbg!(part_2);
}


fn questions_answered_by_group(group_str: &str) -> usize {
	use std::collections::HashSet;

	group_str.chars()
		.filter(char::is_ascii_alphabetic)
		.collect::<HashSet<_>>()
		.len()
}


fn common_questions_answered_by_group(group_str: &str) -> usize {
	let mut question_counts = [0; 26];

	for person in group_str.lines() {
		for ch in person.chars()
			.filter(char::is_ascii_alphabetic)
		{
			let index = ch.to_ascii_lowercase() as usize - 'a' as usize;
			question_counts[index] += 1;
		}
	}

	let people_in_group = group_str.lines().count();

	question_counts.iter()
		.filter(|&&occurances| occurances == people_in_group)
		.count()
}