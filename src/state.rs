use crate::pattern::{Note, Pattern};
use crate::sequence::{Lead, Sequence, LFO};
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
	In(Stage),
	Out(Stage),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SelPatt {
	Prev,
	Next,
}

impl Transition {
	pub fn is_transition_in(&self) -> bool {
		if let Transition::In(_) = self {
			return true;
		}
		false
	}
	pub fn is_transition_out(&self) -> bool {
		if let Transition::Out(_) = self {
			return true;
		}
		false
	}
}

#[derive(Default)]
pub struct State {
	pub running: bool,
	patterns: Vec<Pattern>,
	cur_pattern_id: usize,
	pub sel_patt: Option<SelPatt>,
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
	) -> (
		&mut Box<dyn Sequence + Send>,
		Transition,
		bool,
		bool,
		u8,
		Option<(SelPatt, u8)>,
		Lead,
	) {
		let mut sel_patt: Option<(SelPatt, u8)> = None;
		if step % 96 == 0 {
			if self.next_stage != self.stage
				&& (self.transition == Transition::No || self.transition.is_transition_in())
			{
				self.transition = Transition::Out(self.next_stage);
			} else if self.transition.is_transition_out() {
				self.transition = Transition::In(self.stage);
				self.stage = self.next_stage;
				if self.oh_toggle {
					self.oh = !self.oh;
					self.oh_toggle = false;
				}
				if self.ch_toggle {
					self.ch = !self.ch;
					self.ch_toggle = false;
				}
			} else if self.next_stage == self.stage && self.transition.is_transition_in() {
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

			if let Some(s) = self.sel_patt {
				if match s {
					SelPatt::Prev => self.prev_pattern(),
					SelPatt::Next => self.next_pattern(),
				} {
					let bpm = self.patterns[self.cur_pattern_id].bpm;
					sel_patt = Some((s, bpm));
				}
				self.sel_patt = None;
			}
		}
		let root = self.get_cur_root();

		let sequence =
			self.patterns[self.cur_pattern_id].get_sequence(self.cur_seq_id, &self.stage);

		//TODO
		(
			sequence,
			self.transition,
			self.ch,
			self.oh,
			root,
			sel_patt,
			Lead::Acid,
		)
	}

	pub fn set_next_stage(&mut self, stage: &Stage) {
		self.next_stage = *stage;
	}

	fn prev_pattern(&mut self) -> bool {
		if self.cur_pattern_id > 0 {
			self.cur_pattern_id -= 1;
			return true;
		}
		false
	}

	fn next_pattern(&mut self) -> bool {
		if self.cur_pattern_id < self.patterns.len() - 1 {
			self.cur_pattern_id += 1;
			return true;
		}
		false
	}

	fn get_cur_root(&self) -> u8 {
		self.patterns[self.cur_pattern_id].root.get_midi()
	}

	pub fn get_root_note_bpm(&self) -> (Note, u8) {
		let mut i = self.cur_pattern_id;
		if let Some(p) = self.sel_patt {
			match p {
				SelPatt::Prev => {
					if i > 0 {
						i -= 1
					}
				}
				SelPatt::Next => {
					if i < self.patterns.len() - 1 {
						i += 1
					}
				}
			};
		}
		(self.patterns[i].root, self.patterns[i].bpm)
	}
}
