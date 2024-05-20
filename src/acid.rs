use crate::lead::LEAD_CHANNEL;
use crate::log_send;
use crate::sequence::{end_note, start_note};
use midir::MidiOutputConnection;

#[derive(Default, Clone, Copy)]
pub enum Timing {
	Note,
	Tie,
	#[default]
	Rest,
}

use Timing::*;

#[derive(Default)]
pub struct AcidTrig {
	note: (u8, u8),
	vel: u8,
	slide: bool,
	timing: Timing,
}

#[derive(Default)]
pub struct Acid {
	patterns: Vec<Vec<AcidTrig>>,
	cur_id: usize,
	prev_note: (Timing, u8, u8),
}

impl Acid {
	pub fn new() -> Self {
		let pattern_0 = Self::new_pattern(vec![
			((0, 0), 89, false, Note),
			((0, 0), 89, false, Note),
			((0, 1), 127, false, Note),
			((0, 0), 89, false, Note),
			((11, 0), 127, false, Note),
			((0, 0), 89, false, Note),
			((8, 0), 89, false, Note),
			((0, 0), 89, false, Note),
			((3, 0), 127, false, Note),
			((0, 0), 89, false, Rest),
			((0, 0), 89, false, Note),
			((8, 0), 89, false, Note),
			((0, 0), 89, false, Note),
			((11, 0), 89, false, Note),
			((0, 0), 89, false, Note),
			((0, 1), 127, false, Note),
		]);
		Self {
			patterns: vec![pattern_0],
			cur_id: 0,
			prev_note: (Rest, 0, 0),
		}
	}

	pub fn trigger(&mut self, step: u32, conn: &mut MidiOutputConnection, root: u8) {
		if step % 6 == 0 {
			let t = step / 6;
			let cur_trig = t as usize % self.patterns[self.cur_id].len();
			self.prev_note.0 = self.patterns[self.cur_id][cur_trig].timing;
			let cur_note = &self.patterns[self.cur_id][cur_trig];

			if let Tie = cur_note.timing {
			} else {
				match self.prev_note.0 {
					Note | Tie => {
						log_send(
							conn,
							&end_note(LEAD_CHANNEL, self.prev_note.1, self.prev_note.2),
						);
					}
					_ => {}
				}
			}

			let note = root + cur_note.note.0 + cur_note.note.1 * 12;
			match cur_note.timing {
				Note => {
					log_send(conn, &start_note(LEAD_CHANNEL, note, cur_note.vel));
					self.prev_note.1 = note;
					self.prev_note.2 = cur_note.vel;
				}
				_ => {}
			}
		}
	}

	pub fn next_pattern(&mut self) {
		let len = self.patterns.len();
		self.cur_id = (self.cur_id + 1) % len;
	}

	pub fn new_pattern(x: Vec<((u8, u8), u8, bool, Timing)>) -> Vec<AcidTrig> {
		x.iter()
			.map(|u| AcidTrig {
				note: u.0,
				vel: u.1,
				slide: u.2,
				timing: u.3,
			})
			.collect()
	}

	pub fn get_prev_note(&self) -> (u8, u8) {
		(self.prev_note.1, self.prev_note.2)
	}
}
