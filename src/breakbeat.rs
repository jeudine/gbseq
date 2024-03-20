use crate::hh::HH;
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;
use tseq::sequence::{
	control_change, end_note, param_value, start_note, Sequence, CC_SP1_LAYER, LFO, SP1,
};
use tseq::Stage;
use tseq::{log_send, Transition};

#[derive(Copy, Clone, Default)]
pub struct Breakbeat0 {
	hh: HH,
}

impl Sequence for Breakbeat0 {
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
		if t == 0 || t == 48 || t == 78 {
			log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
		}
	}
}
