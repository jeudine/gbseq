use crate::pattern::Pattern;
use crate::sequence::Sequence;
use std::default::Default;

#[derive(Default, PartialEq)]
pub enum Stage {
	#[default]
	Break,
	Drop,
	HighPass,
	Breakbeat,
	BreakToDrop,
	DropToBreak,
}

#[derive(Default)]
pub struct State {
	pub running: bool,
	patterns: Vec<Pattern>,
	cur_pattern_id: usize,
	stage: Stage,
	cur_seq_id: usize,
	transition_end_step: u32,
}

impl State {
	pub fn get_cur_sequence(&mut self, step: u32) -> &Box<dyn Sequence + Send> {
		// If in a transition & transition is over
		if self.is_in_transition() && step > self.transition_end_step {
			self.stage = State::get_next_stage(&self.stage);
		}
		self.patterns[self.cur_pattern_id].get_sequence(self.cur_seq_id, &self.stage)
	}

	fn get_next_stage(stage: &Stage) -> Stage {
		match stage {
			Stage::BreakToDrop => Stage::Drop,
			Stage::DropToBreak => Stage::Break,
			_ => unreachable!(),
		}
	}

	fn is_in_transition(&self) -> bool {
		match self.stage {
			Stage::BreakToDrop => true,
			Stage::DropToBreak => true,
			_ => false,
		}
	}
}
