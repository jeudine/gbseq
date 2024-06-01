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

#[derive(Copy, Clone, Default)]
pub struct Breakbeat0 {
	patterns: [Rythm; 3],
	level: u8,
	ch_prev: bool,
	oh_prev: bool,
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
		_root: u8,
		transition: Transition,
	) {
		let t = step % 96;

		if t == 0 && transition.is_transition_in() {
			log_send(
				conn,
				&control_change(channel_id, cc_parameter(CC_LAYER, 0), LAYER_ARRAY[2]),
			);
			let pattern = Rythm::compute_euclidean_rythm(rng, &vec![]);
			self.patterns[0] = pattern;
			self.level = 0;
			self.ch_prev = false;
			self.oh_prev = false;
		}

		if ch && !self.ch_prev {}

		// Kicks
		if t == 0 || t == 36 {
			if let Transition::Out(Stage::Drop) = transition {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.3)));
			} else {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
			}
			return;
		}

		if let Transition::Out(Stage::Drop) = transition {
			if t == 84 {
				log_send(
					conn,
					&control_change(channel_id, cc_parameter(CC_LAYER, 0), LAYER_ARRAY[0]),
				);
				log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
			}
		}

		// Percusions
		if step % 6 == 0 {
			let t = step / 6;
			let t = t as usize % NB_TRIGS;
			for (i, p) in self.patterns.iter().enumerate() {
				if p.trigs[t] {
					log_send(conn, &start_note(channel_id, SP_ARRAY[i], param_value(0.0)));
				}
				if i as u8 >= self.level {
					break;
				}
			}
		}
	}
}

impl Rythm {
	fn compute_euclidean_rythm(rng: &mut ThreadRng, existing_k: &Vec<u8>) -> Self {
		let mut _k = rng.gen_range(8..=24);

		// We want a new euclidean rythm
		let mut found = true;
		while found {
			found = false;
			for e_k in existing_k {
				if _k == *e_k {
					found = true;
					_k -= 1;
					break;
				}
			}
		}
		let (k, compl) = if _k > NB_TRIGS as u8 / 2 {
			(NB_TRIGS as u8 - _k, true)
		} else {
			(_k, false)
		};

		let mut mat: [[bool; NB_TRIGS]; NB_TRIGS] = [[false; NB_TRIGS]; NB_TRIGS];

		// Initialize the Matrix & len
		for i in 0..k as usize {
			mat[i][0] = true;
		}

		let mut mat_len: [usize; NB_TRIGS] = [0; NB_TRIGS];
		mat_len[0] = NB_TRIGS;

		// Compute the rythm
		let mut a = NB_TRIGS as u8;
		let mut b = k;
		let mut last_line = 1;

		while b > 0 {
			loop {
				// Move the most right b columns
				let len = mat_len[0];
				let mut moved_elems: [usize; NB_TRIGS] = [0; NB_TRIGS];
				let mut new_last_line = last_line;

				for i in 0..b as usize {
					let col = len - b as usize + i as usize;
					for j in 0..last_line {
						if mat_len[j] > col {
							mat[i][last_line + j] = mat[col][j];
							moved_elems[j] += 1;
							mat_len[last_line + j] += 1;
							if last_line + j + 1 > new_last_line {
								new_last_line = last_line + j + 1;
							}
						} else {
							break;
						}
					}
				}

				for i in 0..last_line {
					mat_len[i] -= moved_elems[i];
				}
				last_line = new_last_line;

				let max_len = mat_len[0];
				let mut second_len = 0;
				for i in 0..last_line {
					if mat_len[i] < max_len {
						second_len = mat_len[i];
						break;
					}
				}
				/*
				//DEBUG
				println!("[{} {}]", a, b);
				for i in 0..last_line {
					for j in 0..mat_len[i] {
						print!(
							"{} ",
							match mat[j][i] {
								true => 1,
								false => 0,
							}
						);
					}
					println!("");
				}
				*/

				if second_len + b as usize > max_len || max_len == b as usize {
					break;
				}
			}

			let r = a % b;
			a = b;
			b = r;
		}

		let nb_col = mat_len[0];

		let mut seq: [bool; NB_TRIGS] = [false; NB_TRIGS];

		let mut iter = 0;

		for i in 0..nb_col {
			let mut j = 0;
			while j < NB_TRIGS && mat_len[j] > i {
				seq[iter] = compl ^ mat[i][j];
				iter += 1;
				j += 1;
			}
		}

		Self { trigs: seq, k: _k }
	}
}
