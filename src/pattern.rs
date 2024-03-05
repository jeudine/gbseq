use crate::sequence::Sequence;

pub struct Pattern {
	pub bpm: Vec<u8>,
	pub s_break: Vec<Box<dyn Sequence>>,
	pub s_drop: Vec<Box<dyn Sequence>>,
	pub s_high_pass: Vec<Box<dyn Sequence>>,
	pub s_breakbeat: Vec<Box<dyn Sequence>>,
	pub break_to_drop: Vec<(Box<dyn Sequence>, u32)>,
}
