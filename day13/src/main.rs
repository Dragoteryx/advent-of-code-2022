mod parse;

mod packet;
use packet::*;

fn right_order([left, right]: &PacketPair) -> bool {
	left < right
}

fn organize_packets(packets: Vec<PacketPair>) -> Vec<Packet> {
	let mut packets: Vec<_> = packets.concat();
	packets.extend([
		vec![PacketData::Integer(2)],
		vec![PacketData::Integer(6)]
	]);

	packets.sort();
	packets
}

fn is_divider(packet: &Packet) -> bool {
	matches!(packet.as_slice(), [PacketData::Integer(2 | 6)])
}

fn part1(pairs: &[PacketPair]) -> usize {
	pairs.iter()
		.enumerate()
		.map(|(id, pair)| (id + 1, pair))
		.filter(|(_, pair)| right_order(pair))
		.map(|(id, _)| id)
		.sum()
}

fn part2(packets: &[Packet]) -> usize {
	packets.iter()
		.enumerate()
		.map(|(id, packet)| (id + 1, packet))
		.filter(|(_, packet)| is_divider(packet))
		.map(|(id, _)| id)
		.product()
}

fn main() {
	let pairs = parse::input(include_str!("real_input.txt"));

	println!("--- day 13 ---");
	println!("part 1 => {}", part1(&pairs));
	println!("part 2 => {}", part2(&organize_packets(pairs)));
}

#[test]
fn test() {
	let pairs = parse::input(include_str!("test_input.txt"));
	assert_eq!(part1(&pairs), 13);
	assert_eq!(part2(&organize_packets(pairs)), 140);
}