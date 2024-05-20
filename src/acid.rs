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
	prev_note: Timing,
}

impl Acid {
	pub fn new() -> Self {
		let pattern_0 = Self::new_pattern(vec![
			((0, 0), 89, false, Note),
			((1, 0), 89, false, Note),
			((2, 0), 89, false, Note),
			((3, 0), 89, false, Note),
			((4, 0), 89, false, Note),
			((5, 0), 89, false, Note),
			((0, 0), 89, false, Note),
			((0, 0), 89, false, Note),
			((0, 0), 89, false, Note),
			((0, 0), 89, false, Note),
			((0, 0), 89, false, Note),
			((0, 0), 89, false, Note),
			((0, 0), 89, false, Note),
			((0, 0), 89, false, Note),
			((0, 0), 89, false, Note),
			((0, 0), 89, false, Note),
		]);
		Self {
			patterns: vec![pattern_0],
			cur_id: 0,
			prev_note: Rest,
		}
	}

	pub fn trigger(&self, step: u32, conn: &mut MidiOutputConnection, root: u8) {
		if step % 6 == 0 {
			let t = step / 6;
			let cur_pattern = &self.patterns[self.cur_id];
			let cur_trig = t as usize % cur_pattern.len();
			let cur_note = &self.patterns[self.cur_id][cur_trig];

			if let Tie = cur_note.timing {
			} else {
				match self.prev_note {
					Note | Tie => {
						log_send(conn, &end_note(LEAD_CHANNEL, root, 100));
					}
					_ => {}
				}
			}

			match cur_note.timing {
				Note => {
					log_send(
						conn,
						&start_note(
							LEAD_CHANNEL,
							root + cur_note.note.0 + cur_note.note.1 * 12,
							100,
						),
					);
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
}
