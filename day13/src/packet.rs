use std::cmp::Ordering;

pub type Packet = Vec<PacketData>;

pub type PacketPair = [Packet; 2];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PacketData {
	List(Packet),
	Integer(u8)
}

impl PartialOrd for PacketData {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for PacketData {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Self::Integer(i1), Self::Integer(i2)) => i1.cmp(i2),
			(Self::List(l1), Self::List(l2)) => l1.cmp(l2),
			(Self::Integer(i), Self::List(l2)) => {
				let l1 = vec![PacketData::Integer(*i)];
				l1.cmp(l2)
			}
			(Self::List(l1), Self::Integer(i)) => {
				let l2 = vec![PacketData::Integer(*i)];
				l1.cmp(&l2)
			}
		}
	}
}