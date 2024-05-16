use midir::MidiOutputConnection;
use tseq::log_send;
use tseq::sequence::{end_note, param_value, start_note, Sequence};

pub enum Timing {
	Note,
	Tie,
	Rest,
}

use Timing::*;

pub struct AcidTrig {
	note: (u8, u8),
	vel: u8,
	slide: bool,
	timing: Timing,
}

pub const ACID_CHANNEL: u8 = 3;

pub struct Acid {
	patterns: Vec<Vec<AcidTrig>>,
	cur_id: usize,
	prev_note: Timing,
}

impl Acid {
	pub fn new() -> Self {
		Self {
			patterns: vec![],
			cur_id: 0,
			prev_note: Rest,
		}
	}

	pub fn trigger(&self, step: u32, conn: &mut MidiOutputConnection, root: u8) {
		if step % 6 == 0 {
			let t = step / 6;
			match self.prev_note {
				Note => log_send(conn, &end_note(ACID_CHANNEL, root, 100)),
				_ => { /*TODO*/ }
			}
			let cur_pattern = &self.patterns[self.cur_id];
			let cur_trig = t as usize % cur_pattern.len();
			let cur_note = &self.patterns[self.cur_id][cur_trig];
			match cur_note.timing {
				Note => log_send(
					conn,
					&start_note(
						ACID_CHANNEL,
						root + cur_note.note.0 + cur_note.note.1 * 12,
						100,
					),
				),
				_ => { /*TODO*/ }
			}
		}
	}
}
