use crate::hh::HH;
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;
use tseq::sequence::{
	cc_parameter, control_change, param_value, start_note, Sequence, CC_FREEZE, CC_LAYER,
	CC_LENGTH, CC_LEVEL, LFO, SP1, SP2, SP3, SP4,
};
use tseq::Stage;
use tseq::{log_send, Transition};

const TRIG_PROBA: f64 = 0.7;
const FREEZE_PROBA: f64 = 0.7;

const SP_ARRAY: [u8; 3] = [SP2, SP3, SP4];
const LAYER_ARRAY: [u8; 3] = [0x00, 0x40, 0x60];

#[derive(Copy, Clone, Default)]
struct Trig {
	sp: u8,
	layer: u8,
}

#[derive(Copy, Clone, Default)]
pub struct Breakbeat0 {
	hh: HH,
	trigs: [Option<Trig>; 16],
	frozen: Option<(u8, u32)>, // (sp, step unfreeze)
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
			log_send(
				conn,
				&control_change(channel_id, cc_parameter(CC_LEVEL, 0), 90),
			);
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
			log_send(
				conn,
				&control_change(channel_id, cc_parameter(CC_LAYER, 0), 0),
			);
			log_send(
				conn,
				&control_change(channel_id, cc_parameter(CC_LENGTH, 0), 63),
			);
			log_send(
				conn,
				&control_change(channel_id, cc_parameter(CC_LENGTH, 1), 31),
			);
			log_send(
				conn,
				&control_change(channel_id, cc_parameter(CC_LENGTH, 2), 31),
			);
			log_send(
				conn,
				&control_change(channel_id, cc_parameter(CC_LENGTH, 3), 31),
			);
		}

		if t == 95 && transition.is_transition_out() {
			self.frozen.inspect(|f| {
				log_send(
					conn,
					&control_change(channel_id, cc_parameter(CC_FREEZE, f.0), 63),
				)
			});
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

		if t % 6 == 0 && !Self::no_perc(transition, t) {
			if let Some(f) = self.frozen {
				if f.1 >= step {
					log_send(
						conn,
						&control_change(channel_id, cc_parameter(CC_FREEZE, f.0), 63),
					);
				}
				self.frozen = None;
			}

			if self.frozen.is_none() && rng.gen_bool(FREEZE_PROBA) {
				let sp = rng.gen_range(1..=3);
				let val = rng.gen_range(0..=127);
				let t = rng.gen_range(1..=3);

				self.frozen = Some((sp, step + t * 24));
				log_send(
					conn,
					&control_change(channel_id, cc_parameter(CC_FREEZE, sp), val),
				);
			}

			let i = t / 6;
			if let Some(t) = self.trigs[i as usize] {
				log_send(
					conn,
					&control_change(
						channel_id,
						cc_parameter(CC_LAYER, t.sp + 1),
						LAYER_ARRAY[t.layer as usize],
					),
				);
				if let Transition::Out(Stage::Drop) = transition {
					log_send(
						conn,
						&start_note(
							channel_id,
							SP_ARRAY[t.sp as usize],
							param_value(i as f32 / 12.0),
						),
					);
				} else {
					log_send(
						conn,
						&start_note(channel_id, SP_ARRAY[t.sp as usize], param_value(0.0)),
					);
				}
			}
		}

		if ch {
			self.hh.trigger_ch_dnb(step, conn, root);
		}
	}
}

impl Breakbeat0 {
	fn no_perc(transition: Transition, t: u32) -> bool {
		if let Transition::Out(Stage::Drop) = transition {
			if t >= 72 {
				return true;
			}
		}
		false
	}
}
