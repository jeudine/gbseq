use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;
use tseq::log_send;
use tseq::sequence::{end_note, param_value, start_note, Sequence, SP1};

const SKIPPED_PROBA: f64 = 0.2;
const DOUBLED_PROBA: f64 = 0.2;

#[derive(Copy, Clone, Default)]
pub struct Drop0 {
	is_skipped: bool,
	is_doubled: bool,
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
	) {
		let t = step % 96;
		if t == 0 || t == 24 || t == 48 {
			log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
		}

		if t == 12 {
			self.is_doubled = rng.gen_bool(DOUBLED_PROBA);
			if self.is_doubled {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
			}
		}

		if t == 72 {
			self.is_skipped = rng.gen_bool(SKIPPED_PROBA);
			if !self.is_skipped {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
			}
		}

		/*
		if t == 6
			|| t == 30 || t == 54
			|| (t == 18 && self.is_doubled)
			|| (t == 78 && !self.is_skipped)
		{
			log_send(conn, &end_note(channel_id, SP1, param_value(0.0)));
		}
		*/
	}
}
