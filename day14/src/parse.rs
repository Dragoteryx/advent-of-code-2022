type NomResult<'s, T> = nom::IResult<&'s str, T>;

fn point(input: &str) -> NomResult<(usize, usize)> {
	use nom::character::complete::{char, u16};
	use nom::sequence::separated_pair;
	use nom::combinator::map;

	separated_pair(
		map(u16, |n| n as usize),
		char(','),
		map(u16, |n| n as usize)
	)(input)
}

fn path(input: &str) -> NomResult<Vec<(usize, usize)>> {
	use nom::multi::separated_list1;
	use nom::bytes::complete::tag;

	separated_list1(
		tag(" -> "),
		point
	)(input)
}

fn paths(input: &str) -> NomResult<Vec<Vec<(usize, usize)>>> {
	use nom::character::complete::newline;
	use nom::multi::separated_list0;

	separated_list0(
		newline,
		path
	)(input)
}

pub(crate) fn input(input: &str) -> Vec<Vec<(usize, usize)>> {
	use nom::sequence::terminated;
	use nom::combinator::eof;

	let Ok(("", paths)) = terminated(paths, eof)(input) else {
		panic!("invalid input");
	};

	paths
}