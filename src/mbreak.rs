use crate::hh::HH;
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use tseq::sequence::{
	cc_parameter, control_change, param_value, start_note, Sequence, CC_LAYER, CC_LENGTH, SP1,
};
use tseq::{log_send, Stage, Transition};

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
		let t = step % 96;

		let mut no_hh = false;

		if t == 0 && transition.is_transition_in() {
			log_send(
				conn,
				&control_change(channel_id, cc_parameter(CC_LAYER, 0), 0),
			);
			log_send(
				conn,
				&control_change(channel_id, cc_parameter(CC_LENGTH, 0), 127),
			);
		}
		if let Transition::Out(Stage::Drop) = transition {
			if t == 0 {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.6)));
			} else if t == 24 {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.5)));
			} else if t == 48 {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.4)));
			} else if t == 84 {
				log_send(
					conn,
					&control_change(channel_id, cc_parameter(CC_LAYER, 0), 1 << 6),
				);

				log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
			}

			if t >= 72 {
				no_hh = true;
			}
		} else if let Transition::Out(Stage::Breakbeat) = transition {
			if t >= 72 {
				no_hh = true;
			}
		}

		if oh && !no_hh {
			self.hh.trigger_oh(step, conn, root, rng);
		}
		if ch && !no_hh {
			self.hh.trigger_ch(step, conn, root);
		}
	}
}
