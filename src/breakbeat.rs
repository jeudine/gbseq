use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;
use tseq::sequence::{
	cc_parameter, control_change, param_value, start_note, Sequence, CC_FREEZE, CC_LAYER,
	CC_LENGTH, CC_LEVEL, SP1, SP2, SP3, SP4,
};
use tseq::Stage;
use tseq::{log_send, Transition};

const TRIG_PROBA: f64 = 0.7;
const FREEZE_PROBA: f64 = 0.7;

const SP_ARRAY: [u8; 3] = [SP2, SP3, SP4];
const LAYER_ARRAY: [u8; 3] = [0x00, 0x40, 0x60];
const NB_TRIGS: usize = 32;

#[derive(Copy, Clone, Default)]
struct Rythm {
	trigs: [bool; NB_TRIGS],
	k: u8,
}

#[derive(Clone, Default)]
pub struct Breakbeat0 {
	patterns: Vec<(Rythm, u8, u8)>, // (rythm, sp, layer)
	level: u8,
}

// TODO: try adding LFO on some effects
impl Sequence for Breakbeat0 {
	fn run(
		&mut self,
		step: u32,
		conn: &mut MidiOutputConnection,
		channel_id: u8,
		rng: &mut ThreadRng,
		_oh: bool,
		_ch: bool,
		_root: u8,
		transition: Transition,
	) {
		let t = step % 96;

		if t == 0 && transition.is_transition_in() {
			self.level = 0;
		}

		if t == 0 || t == 36 {
			if let Transition::Out(Stage::Drop) = transition {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.3)));
			} else {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
			}
		}

		if let Transition::Out(Stage::Drop) = transition {
			if t == 84 {
				log_send(
					conn,
					&control_change(channel_id, cc_parameter(CC_LENGTH, 0), 127),
				);
				log_send(
					conn,
					&control_change(channel_id, cc_parameter(CC_LAYER, 0), 1 << 6),
				);
				log_send(
					conn,
					&control_change(channel_id, cc_parameter(CC_LEVEL, 0), 63),
				);
				log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
			}
		}
	}
}

impl Rythm {
	fn compute_euclidean_rythm(rng: &mut ThreadRng, existing_k: &Vec<u8>) -> Self {
		let mut k = rng.gen_range(8..=24);

		// We want a new euclidean rythm
		let mut found = true;
		while found {
			found = false;
			for e_k in existing_k {
				if k == *e_k {
					found = true;
					k -= 1;
					break;
				}
			}
		}

		let mut mat: [[bool; NB_TRIGS]; NB_TRIGS] = [[false; NB_TRIGS]; NB_TRIGS];

		// Initialize the Matrix & len
		for i in 0..k as usize {
			mat[i][0] = true;
		}

		let mut mat_len: [usize; NB_TRIGS] = [0; NB_TRIGS];
		mat_len[0] = NB_TRIGS;

		// Compute the rythm

		let trigs: [bool; NB_TRIGS] = [false; NB_TRIGS];
		Self { trigs, k }
	}
}
