use crate::pattern::Pattern;
use crate::sequence::Sequence;
use std::default::Default;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
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
	next_stage: Stage,
	cur_seq_id: usize,
	transition_end_step: u32,
}

impl State {
	pub fn new(patterns: Vec<Pattern>) -> Self {
		let mut state = State::default();
		state.patterns = patterns;
		state
	}

	pub fn get_cur_sequence(&mut self, step: u32) -> &Box<dyn Sequence + Send> {
		// Enter in a transition
		if step % 96 == 0 && self.next_stage != self.stage {
			self.stage = self.next_stage;
			self.transition_end_step = self.patterns[self.cur_pattern_id]
				.get_transition_len(self.cur_seq_id, &self.stage)
				+ step;
		}

		// If in a transition & transition is over
		if self.is_in_transition() && step >= self.transition_end_step {
			self.stage = State::get_next_stage(&self.stage);
			self.next_stage = self.stage;
		}

		//TODO: add debug mode
		//println!("{}: {:?}", step, self.stage);
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

	// If in a transition, the next stage won't be set
	pub fn set_next_stage(&mut self, stage: &Stage) {
		match &self.stage {
			Stage::Break => match stage {
				Stage::Drop => self.next_stage = Stage::BreakToDrop,
				Stage::HighPass => {}
				Stage::Breakbeat => {}
				_ => {}
			},
			Stage::Drop => match stage {
				Stage::Break => self.next_stage = Stage::DropToBreak,
				Stage::HighPass => {}
				Stage::Breakbeat => {}
				_ => {}
			},
			Stage::HighPass => match stage {
				Stage::Break => {}
				Stage::Drop => {}
				Stage::HighPass => {}
				Stage::Breakbeat => {}
				_ => {}
			},
			Stage::Breakbeat => match stage {
				Stage::Break => {}
				Stage::Drop => {}
				Stage::HighPass => {}
				Stage::Breakbeat => {}
				_ => {}
			},
			_ => {}
		}
	}
}
