#![deny(rust_2018_idioms, future_incompatible)]
#![feature(type_ascription, str_split_once)]

use std::collections::HashMap;


fn main() {
	let data = include_str!("input.txt")
		.split("\n\n")
		.map(parse_passport)
		.collect(): Vec<_>;

	let part_1 = data.iter()
		.map(|pass| part_1_valid(pass) as usize)
		.sum(): usize;

	dbg!(part_1);

	let part_2 = data.iter()
		.map(|pass| part_2_valid(pass) as usize)
		.sum(): usize;

	dbg!(part_2);
}

#[derive(Debug)]
struct Passport(HashMap<Field, String>);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Field {
    BirthYear, // byr
    IssueYear, // iyr
    ExpirationYear, // eyr
    Height, // hgt
    HairColor, // hcl
    EyeColor, // ecl
    PassportId, // pid
    CountryId, // cid
}

fn parse_passport(input: &str) -> Passport {
	let map = input.split_whitespace()
		.map(parse_passport_field)
		.collect();

	Passport(map)
}

fn parse_passport_field(input: &str) -> (Field, String) {
	let (field_name, value) = input.split_once(':')
		.expect("Invalid field");

	let field = match field_name {
		"byr" => Field::BirthYear,
		"iyr" => Field::IssueYear,
		"eyr" => Field::ExpirationYear,
		"hgt" => Field::Height,
		"hcl" => Field::HairColor,
		"ecl" => Field::EyeColor,
		"pid" => Field::PassportId,
		"cid" => Field::CountryId,
		_ => panic!("Invalid field name '{}'", field_name),
	};

	(field, value.into())
}


fn part_1_valid(Passport(fields): &Passport) -> bool {
	let required_fields = [
		Field::BirthYear,
		Field::IssueYear,
		Field::ExpirationYear,
		Field::Height,
		Field::HairColor,
		Field::EyeColor,
		Field::PassportId,
	];

	for field in required_fields.iter() {
		if !fields.contains_key(field) {
			return false
		}
	}

	true
}


fn validate_year(year_str: &str, lb: usize, ub: usize) -> bool {
	let year = match year_str.parse() {
		Ok(y) => y,
		Err(_) => return false
	};

	lb <= year && year <= ub
}

fn validate_height(height_str: &str) -> bool {
	if height_str.len() <= 2 { return false }

	let value_str = &height_str[..height_str.len()-2];

	let value: usize = match value_str.parse() { Ok(v) => v, Err(_) => return false };
	let unit = &height_str[height_str.len()-2..];

	match unit {
		"in" => 59 <= value && value <= 76,
		"cm" => 150 <= value && value <= 193,
		_ => false
	}
}

fn validate_hair_color(col_str: &str) -> bool {
	if !col_str.starts_with('#') { return false }

	col_str.starts_with('#')
		&& col_str[1..].chars().all(|c| c.is_ascii_hexdigit())
}

fn validate_eye_color(col_str: &str) -> bool {
	["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&col_str)
}

fn validate_passport_id(id_str: &str) -> bool {
	id_str.len() == 9
		&& id_str.chars().all(|c| c.is_ascii_digit())
}


fn part_2_valid(passport: &Passport) -> bool {
	if !part_1_valid(passport) {
		return false
	}

	let Passport(fields) = passport;

	validate_year(&fields[&Field::BirthYear], 1920, 2002)
	&& validate_year(&fields[&Field::IssueYear], 2010, 2020)
	&& validate_year(&fields[&Field::ExpirationYear], 2020, 2030)
	&& validate_height(&fields[&Field::Height])
	&& validate_hair_color(&fields[&Field::HairColor])
	&& validate_eye_color(&fields[&Field::EyeColor])
	&& validate_passport_id(&fields[&Field::PassportId])
}