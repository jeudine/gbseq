use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;
use tseq::log_send;
use tseq::sequence::{end_note, param_value, start_note, Sequence, SP2, SP3};

#[derive(Copy, Clone, Default)]
pub struct HH {}

impl HH {
	pub fn trigger_ch(&self, step: u32, conn: &mut MidiOutputConnection, channel_id: u8) {
		if step % 6 == 0 {
			log_send(conn, &start_note(channel_id, SP3, param_value(0.0)));
		}
	}
}
