use crate::hh::HH;
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;
use tseq::sequence::{
	control_change, end_note, param_value, start_note, Sequence, CC_SP1_LAYER, CC_SP1_LENGTH,
	CC_SP2_LAYER, CC_SP2_LENGTH, CC_SP3_LAYER, CC_SP3_LENGTH, CC_SP4_LAYER, CC_SP4_LENGTH, LFO,
	SP1, SP2, SP3, SP4,
};
use tseq::Stage;
use tseq::{log_send, Transition};

const TRIG_PROBA: f64 = 0.7;

const SP_ARRAY: [u8; 3] = [SP2, SP3, SP4];
const LAYER_ARRAY: [u8; 3] = [0x00, 0x40, 0x60];
const CC_LAYER_ARRAY: [u8; 3] = [CC_SP2_LAYER, CC_SP3_LAYER, CC_SP4_LAYER];

#[derive(Copy, Clone, Default)]
struct Trig {
	sp: u8,
	layer: u8,
}

#[derive(Copy, Clone, Default)]
pub struct Breakbeat0 {
	hh: HH,
	trigs: [Option<Trig>; 16],
}

// TODO: try adding LFO on some effects
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

		if t == 0 && transition.is_transition_in() {
			for i in 0..16 {
				self.trigs[i] = if rng.gen_bool(TRIG_PROBA) {
					let sp = rng.gen_range(0..3);
					let layer = rng.gen_range(0..3);
					let trig = Trig { sp, layer };
					Some(trig)
				} else {
					None
				}
			}
			log_send(conn, &control_change(channel_id, CC_SP1_LENGTH, 31));
			log_send(conn, &control_change(channel_id, CC_SP2_LENGTH, 31));
			log_send(conn, &control_change(channel_id, CC_SP3_LENGTH, 31));
			log_send(conn, &control_change(channel_id, CC_SP3_LENGTH, 31));
		}

		//if t == 0 || t == 36 {
		if t == 0 || t == 24 || t == 48 || t == 72 {
			log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
		}

		if t % 6 == 0 {
			let i = t / 6;
			if let Some(t) = self.trigs[i as usize] {
				log_send(
					conn,
					&control_change(
						channel_id,
						CC_LAYER_ARRAY[t.sp as usize],
						LAYER_ARRAY[t.layer as usize],
					),
				);
				log_send(
					conn,
					&start_note(channel_id, SP_ARRAY[t.sp as usize], param_value(0.0)),
				);
			}
		}
	}
}
