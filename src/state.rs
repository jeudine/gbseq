use crate::pattern::Pattern;
use crate::sequence::{Sequence, LFO};
use rand::rngs::ThreadRng;
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
	pub oh: bool,
	pub ch: bool,
	pub oh_toggle: bool,
	pub ch_toggle: bool,
	pub oh_lfo: LFO,
	pub ch_lfo: LFO,
}

impl State {
	pub fn new(patterns: Vec<Pattern>) -> Self {
		let mut state = State::default();
		state.patterns = patterns;
		state
	}

	pub fn get_cur_sequence(
		&mut self,
		step: u32,
		rng: &mut ThreadRng,
	) -> &mut Box<dyn Sequence + Send> {
		// Enter in a transition
		if step % 96 == 0 {
			if self.next_stage != self.stage {
				self.stage = self.next_stage;
				self.transition_end_step = self.patterns[self.cur_pattern_id]
					.get_transition_len(self.cur_seq_id, &self.stage)
					+ step;
			} else {
				if self.oh_toggle {
					self.oh = !self.oh;
					self.oh_toggle = false;
					if self.oh {
						self.oh_lfo
							.set_rng(&(vec![1.0, 2.0, 4.0])[..], 0.2..0.4, rng);
					}
				}
				if self.ch_toggle {
					self.ch = !self.ch;
					self.ch_toggle = false;
					if self.ch {
						self.ch_lfo
							.set_rng(&(vec![1.0, 2.0, 4.0])[..], 0.2..0.4, rng);
					}
				}
			}
		}

		// If in a transition & transition is over
		if self.is_in_transition() && step >= self.transition_end_step {
			self.stage = State::get_next_stage(&self.stage);
			self.next_stage = self.stage;
			if self.oh_toggle {
				self.oh = !self.oh;
				self.oh_toggle = false;
				if self.oh {
					self.oh_lfo
						.set_rng(&(vec![1.0, 2.0, 4.0])[..], 0.2..0.4, rng);
				}
			}
			if self.ch_toggle {
				self.ch = !self.ch;
				self.ch_toggle = false;
				if self.ch {
					self.ch_lfo
						.set_rng(&(vec![1.0, 2.0, 4.0])[..], 0.2..0.4, rng);
				}
			}
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
