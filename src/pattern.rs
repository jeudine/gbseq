use crate::sequence::Sequence;
use crate::state::Stage;

pub struct Pattern {
	pub bpm: u8,
	pub root: Note,
	pub s_break: Vec<Box<dyn Sequence + Send>>,
	pub s_drop: Vec<Box<dyn Sequence + Send>>,
	pub s_high_pass: Vec<Box<dyn Sequence + Send>>,
	pub s_breakbeat: Vec<Box<dyn Sequence + Send>>,
	pub break_to_drop: Vec<(Box<dyn Sequence + Send>, u32)>,
	pub drop_to_break: Vec<(Box<dyn Sequence + Send>, u32)>,
}

impl Pattern {
	pub fn get_sequence(&mut self, seq_id: usize, stage: &Stage) -> &mut Box<dyn Sequence + Send> {
		match stage {
			Stage::Break => &mut self.s_break[seq_id],
			Stage::Drop => &mut self.s_drop[seq_id],
			Stage::HighPass => &mut self.s_high_pass[seq_id],
			Stage::Breakbeat => &mut self.s_breakbeat[seq_id],
			Stage::BreakToDrop => &mut self.break_to_drop[seq_id].0,
			Stage::DropToBreak => &mut self.drop_to_break[seq_id].0,
		}
	}

	pub fn get_transition_len(&self, seq_id: usize, stage: &Stage) -> u32 {
		match stage {
			Stage::BreakToDrop => self.break_to_drop[seq_id].1,
			Stage::DropToBreak => self.drop_to_break[seq_id].1,
			_ => unreachable!(),
		}
	}
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Note {
	#[default]
	C,
	CS,
	D,
	DS,
	E,
	F,
	FS,
	G,
	GS,
	A,
	AS,
	B,
}

impl Note {
	pub fn get_midi(&self) -> u8 {
		match self {
			Note::C => 60,
			Note::CS => 61,
			Note::D => 62,
			Note::DS => 63,
			Note::E => 64,
			Note::F => 65,
			Note::FS => 54,
			Note::G => 55,
			Note::GS => 56,
			Note::A => 57,
			Note::AS => 58,
			Note::B => 59,
		}
	}
}
