use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;
use tseq::log_send;
use tseq::sequence::{end_note, param_value, start_note, Sequence, LFO, SP2, SP3};

const DOUBLED_PROBA: f64 = 0.2;

#[derive(Copy, Clone, Default)]
pub struct HH {}

impl HH {
	pub fn trigger_ch(
		&self,
		step: u32,
		conn: &mut MidiOutputConnection,
		channel_id: u8,
		lfo: &LFO,
	) {
		if step % 6 == 0 {
			log_send(
				conn,
				&start_note(channel_id, SP3, param_value(lfo.get_val(step))),
			);
		}
	}

	pub fn trigger_oh(
		&self,
		step: u32,
		conn: &mut MidiOutputConnection,
		channel_id: u8,
		rng: &mut ThreadRng,
		lfo: &LFO,
	) {
		if step % 24 == 12 || (step & 96 == 72 && rng.gen_bool(DOUBLED_PROBA)) {
			log_send(
				conn,
				&start_note(channel_id, SP2, param_value(lfo.get_val(step))),
			);
		}
	}
}
