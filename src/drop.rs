use crate::hh::HH;
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;
use tseq::log_send;
use tseq::sequence::{end_note, param_value, start_note, Sequence, LFO, SP1};

const SKIPPED_PROBA: f64 = 0.2;
const DOUBLED_PROBA: f64 = 0.2;

#[derive(Copy, Clone, Default)]
pub struct Drop0 {
	hh: HH,
}

impl Sequence for Drop0 {
	fn run(
		&mut self,
		step: u32,
		conn: &mut MidiOutputConnection,
		channel_id: u8,
		rng: &mut ThreadRng,
		oh: bool,
		ch: bool,
		oh_lfo: &LFO,
		ch_lfo: &LFO,
	) {
		let t = step % 96;
		if t == 0 || t == 24 || t == 48 {
			log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
		}

		if t == 12 && rng.gen_bool(DOUBLED_PROBA) {
			log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
		}

		if t == 72 && rng.gen_bool(SKIPPED_PROBA) {
			log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
		}

		if oh {
			self.hh.trigger_oh(step, conn, channel_id, rng, oh_lfo);
		}
		if ch {
			self.hh.trigger_ch(step, conn, channel_id, ch_lfo);
		}
	}
}
