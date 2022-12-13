use nom::IResult;
use super::*;

fn monke_id(input: &str) -> IResult<&str, u8> {
	use nom::character::complete::u8;
	use nom::bytes::complete::tag;
	use nom::sequence::delimited;
	
	delimited(
		tag("Monkey "),
		u8,
		tag(":")
	)(input)
}

fn monke_items(input: &str) -> IResult<&str, Vec<u128>> {
	use nom::character::complete::u128;
	use nom::multi::separated_list1;
	use nom::bytes::complete::tag;
	use nom::sequence::preceded;

	preceded(
		tag("Starting items: "),
		separated_list1(
			tag(", "),
			u128
		)
	)(input)
}

fn monke_operation(input: &str) -> IResult<&str, Operation> {
	use nom::character::complete::u128;
	use nom::bytes::complete::tag;
	use nom::sequence::preceded;
	use nom::combinator::map;
	use nom::branch::alt;
	
	preceded(
		tag("Operation: new = "),
		alt((
			map(preceded(tag("old * "), u128), Operation::Mult),
			map(preceded(tag("old + "), u128), Operation::Add),
			map(tag("old * old"), |_| Operation::Square)
		))
	)(input)
}

fn monke_test(input: &str) -> IResult<&str, u128> {
	use nom::character::complete::u128;
	use nom::bytes::complete::tag;
	use nom::sequence::preceded;

	preceded(
		tag("Test: divisible by "),
		u128
	)(input)
}

fn monke_throw_to(input: &str) -> IResult<&str, usize> {
	use nom::character::complete::u8;
	use nom::bytes::complete::tag;
	use nom::sequence::preceded;
	use nom::combinator::map;

	preceded(
		tag("throw to monkey "),
		map(u8, |n| n as usize)
	)(input)
}

fn monke_if_true(input: &str) -> IResult<&str, usize> {
	use nom::bytes::complete::tag;
	use nom::sequence::preceded;

	preceded(
		tag("If true: "),
		monke_throw_to
	)(input)
}

fn monke_if_false(input: &str) -> IResult<&str, usize> {
	use nom::bytes::complete::tag;
	use nom::sequence::preceded;

	preceded(
		tag("If false: "),
		monke_throw_to
	)(input)
}

fn monke(input: &str) -> IResult<&str, Monke> {
	use nom::character::complete::multispace1;
	use nom::sequence::preceded;
	use nom::sequence::tuple;

	let (input, (
		_,
		items,
		operation,
		test,
		if_true,
		if_false
	)) = tuple((
		monke_id,
		preceded(multispace1, monke_items),
		preceded(multispace1, monke_operation),
		preceded(multispace1, monke_test),
		preceded(multispace1, monke_if_true),
		preceded(multispace1, monke_if_false)
	))(input)?;

	Ok((
		input,
		Monke::new(
			items, operation,
			test, if_true,
			if_false
		)
	))
}

fn monkeys(input: &str) -> IResult<&str, Vec<Monke>> {
	use nom::character::complete::line_ending;
	use nom::multi::separated_list0;
	use nom::sequence::pair;

	separated_list0(
		pair(
			line_ending,
			line_ending
		),
		monke
	)(input)
}

pub(crate) fn input(input: &str) -> Vec<Monke> {
	use nom::sequence::terminated;
	use nom::combinator::eof;

	let Ok(("", monkeys)) = terminated(monkeys, eof)(input) else {
		panic!("invalid input");
	};

	monkeys
}