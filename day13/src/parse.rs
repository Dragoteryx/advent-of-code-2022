use nom::IResult;
use super::*;

fn packet(input: &str) -> IResult<&str, Packet> {
	use nom::multi::separated_list0;
	use nom::bytes::complete::tag;
	use nom::sequence::delimited;

	delimited(
		tag("["),
		separated_list0(
			tag(","),
			packet_data
		),
		tag("]")
	)(input)
}

fn packet_data(input: &str) -> IResult<&str, PacketData> {
	use nom::character::complete::u8;
	use nom::combinator::map;
	use nom::branch::alt;

	alt((
		map(u8, PacketData::Integer),
		map(packet, PacketData::List)
	))(input)
}

fn packet_pair(input: &str) -> IResult<&str, PacketPair> {
	use nom::character::complete::line_ending;
	use nom::sequence::separated_pair;
	use nom::combinator::map;

	map(
		separated_pair(
			packet,
			line_ending,
			packet
		),
		|(left, right)| [left, right]
	)(input)
}

fn packets(input: &str) -> IResult<&str, Vec<PacketPair>> {
	use nom::character::complete::line_ending;
	use nom::multi::separated_list0;
	use nom::sequence::pair;

	separated_list0(
		pair(
			line_ending,
			line_ending
		),
		packet_pair
	)(input)
}

pub(crate) fn input(input: &str) -> Vec<PacketPair> {
	use nom::sequence::terminated;
	use nom::combinator::eof;

	let Ok(("", packets)) = terminated(packets, eof)(input) else {
		panic!("invalid input");
	};

	packets
}