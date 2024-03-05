use crate::message::{end_note, param_value, start_note};
use midir::MidiOutputConnection;

pub trait Sequence {
	fn run(&self, step: u32, conn: &mut MidiOutputConnection, channel: u8);
}

pub const SP1: u8 = 48;

pub struct Drop0 {}

impl Sequence for Drop0 {
	fn run(&self, step: u32, conn: &mut MidiOutputConnection, channel: u8) {
		if step % 24 == 0 {
			let _ = conn.send(&start_note(channel, SP1, param_value(0.0)));
		}

		if step % 24 == 18 {
			let _ = conn.send(&end_note(channel, SP1, param_value(0.0)));
		}
	}
}
