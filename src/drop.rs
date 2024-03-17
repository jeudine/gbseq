use crate::hh::HH;
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;
use tseq::sequence::{
	control_change, end_note, param_value, start_note, Sequence, CC_SP1_LAYER, LFO, SP1,
};
use tseq::{log_send, Transition};

const SKIPPED_PROBA: f64 = 0.2;
const DOUBLED_PROBA: f64 = 0.2;

#[derive(Copy, Clone, Default)]
pub struct Drop0 {
	hh: HH,
	skipped: bool,
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
		root: u8,
		transition: Transition,
	) {
		let t = step % 96;

		if t == 0 {
			log_send(conn, &control_change(channel_id, CC_SP1_LAYER, 0));
		}

		if transition == Transition::In {
			if t == 12 {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
			}
		}

		if t == 0 || t == 24 || t == 48 {
			log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
		}

		if t == 12 && rng.gen_bool(DOUBLED_PROBA) {
			log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
		}

		if t == 72 {
			if !rng.gen_bool(SKIPPED_PROBA) {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
				self.skipped = false;
			} else {
				self.skipped = true;
			}
		}

		if t == 84 && self.skipped {
			log_send(conn, &control_change(channel_id, CC_SP1_LAYER, 1 << 6));
			log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
		}

		if oh {
			self.hh.trigger_oh(step, conn, root, rng);
		}
		if ch {
			self.hh.trigger_ch(step, conn, root);
		}
	}
}

#[derive(Copy, Clone, Default)]
pub struct HighPass0 {
	hh: HH,
}

impl Sequence for HighPass0 {
	fn run(
		&mut self,
		step: u32,
		conn: &mut MidiOutputConnection,
		channel_id: u8,
		rng: &mut ThreadRng,
		oh: bool,
		ch: bool,
		root: u8,
		transition: Transition,
	) {
		let t = step % 96;

		if t == 0 {
			log_send(conn, &control_change(channel_id, CC_SP1_LAYER, 0));
		}

		if transition == Transition::In || transition == Transition::No {
			if t == 0 || t == 24 || t == 48 {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.6)));
			} else if t == 12 && rng.gen_bool(DOUBLED_PROBA) {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.6)));
			} else if t == 72 && !rng.gen_bool(SKIPPED_PROBA) {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.6)));
			}
		} else {
			if t == 0 {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.6)));
			} else if t == 24 {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.5)));
			} else if t == 48 {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.4)));
			} else if t == 84 {
				log_send(conn, &control_change(channel_id, CC_SP1_LAYER, 1 << 6));
				log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
			}
		}

		if oh {
			self.hh.trigger_oh(step, conn, root, rng);
		}
		if ch {
			self.hh.trigger_ch(step, conn, root);
		}
	}
}
