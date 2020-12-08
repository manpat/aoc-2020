#![deny(rust_2018_idioms, future_incompatible)]
#![feature(type_ascription, str_split_once)]



use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let list = include_str!("input.txt")
		.lines()
		.map(parse_database_entry)
		.collect(): Result<Vec<_>, _>;

	let list = list?;

	println!("part_1 {}", part_1(&list));
	println!("part_2 {}", part_2(&list));

	Ok(())
}

#[derive(Debug, Copy, Clone)]
struct Rule {
	ch: char,
	lb: usize,
	ub: usize,
}

#[derive(Debug)]
struct PasswordEntry {
	rule: Rule,
	password: String,
}


fn parse_database_entry(line: &str) -> Result<PasswordEntry, Box<dyn Error>> {
	let (rule, password) = line.split_once(':')
		.expect("Invalid input!");

	Ok(PasswordEntry {
		rule: parse_rule(rule.trim())?,
		password: password.trim().into(),
	})
}

fn parse_rule(rule_str: &str) -> Result<Rule, Box<dyn Error>> {
	let (range_str, ch) = rule_str.split_once(' ')
		.expect("Invalid rule syntax!");

	let (lb, ub) = range_str.split_once('-')
		.expect("Invalid range syntax!");

	Ok(Rule {
		ch: ch.chars().next().expect("Missing char in rule"),
		lb: lb.parse()?,
		ub: ub.parse()?,
	})
}


fn part_1(list: &[PasswordEntry]) -> usize {
	fn valid(entry: &&PasswordEntry) -> bool {
		let ch_count = entry.password.chars()
			.filter(|&c| c == entry.rule.ch)
			.count();

		entry.rule.lb <= ch_count && ch_count <= entry.rule.ub
	}

	list.iter().filter(valid).count()
}


fn part_2(list: &[PasswordEntry]) -> usize {
	fn valid(PasswordEntry{password, rule}: &&PasswordEntry) -> bool {
		let Rule{ch, lb, ub} = *rule;
		let ch_l = match password.chars().nth(lb-1) { Some(v) => v, None => return false };
		let ch_u = match password.chars().nth(ub-1) { Some(v) => v, None => return false };

		(ch_l == ch) != (ch_u == ch)
	}

	list.iter().filter(valid).count()
}