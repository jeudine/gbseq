use midir::MidiOutputConnection;
use tseq::log_send;
use tseq::sequence::{end_note, param_value, start_note, Sequence, SP1};

#[derive(Copy, Clone)]
pub struct Drop0 {}

impl Sequence for Drop0 {
	fn run(&self, step: u32, conn: &mut MidiOutputConnection, channel_id: u8) {
		if step % 24 == 0 {
			log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
		}

		if step % 24 == 18 {
			log_send(conn, &end_note(channel_id, SP1, param_value(0.0)));
		}
	}
}
