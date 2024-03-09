use crate::sequence::Sequence;
use crate::state::Stage;

pub struct Pattern {
	pub bpm: u8,
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
