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
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Transition {
	#[default]
	No,
	In,
	Out,
}

#[derive(Default)]
pub struct State {
	pub running: bool,
	patterns: Vec<Pattern>,
	cur_pattern_id: usize,
	stage: Stage,
	next_stage: Stage,
	cur_seq_id: usize,
	oh: bool,
	ch: bool,
	pub oh_toggle: bool,
	pub ch_toggle: bool,
	transition: Transition,
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
	) -> (&mut Box<dyn Sequence + Send>, Transition, bool, bool, u8) {
		if step % 96 == 0 {
			if self.next_stage != self.stage && self.transition == Transition::No {
				self.transition = Transition::Out;
			} else if self.next_stage != self.stage && self.transition == Transition::Out {
				self.transition = Transition::In;
				self.stage = self.next_stage;
				if self.oh_toggle {
					self.oh = !self.oh;
					self.oh_toggle = false;
				}
				if self.ch_toggle {
					self.ch = !self.ch;
					self.ch_toggle = false;
				}
			} else if self.next_stage == self.stage && self.transition == Transition::In {
				self.transition = Transition::No;
			} else if self.next_stage == self.stage && self.transition == Transition::No {
				if self.oh_toggle {
					self.oh = !self.oh;
					self.oh_toggle = false;
				}
				if self.ch_toggle {
					self.ch = !self.ch;
					self.ch_toggle = false;
				}
			}
		}
		let root = self.get_cur_root();
		let sequence =
			self.patterns[self.cur_pattern_id].get_sequence(self.cur_seq_id, &self.stage);

		(sequence, self.transition, self.ch, self.oh, root)
	}

	pub fn set_next_stage(&mut self, stage: &Stage) {
		self.next_stage = *stage;
	}

	fn get_cur_root(&self) -> u8 {
		self.patterns[self.cur_pattern_id].root.get_midi()
	}
}
