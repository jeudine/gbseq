use crate::hh::HH;
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use tseq::sequence::{end_note, param_value, start_note, Sequence, LFO, SP2, SP3};
use tseq::{log_send, Transition};

#[derive(Copy, Clone, Default)]
pub struct Break0 {
	hh: HH,
}

impl Sequence for Break0 {
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
		if oh {
			self.hh.trigger_oh(step, conn, root, rng);
		}
		if ch {
			self.hh.trigger_ch(step, conn, root);
		}
	}
}
