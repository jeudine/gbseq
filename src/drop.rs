use crate::hh::HH;
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::{
	distributions::{Distribution, Standard},
	Rng,
};
use tseq::sequence::{
	cc_parameter, control_change, end_note, param_value, start_note, Sequence, CC_LAYER, CC_LENGTH,
	LFO, SP1,
};
use tseq::Stage;
use tseq::{log_send, Transition};

const SKIPPED_PROBA: f64 = 0.2;
const DOUBLED_PROBA: f64 = 0.2;

#[derive(Copy, Clone, PartialEq)]
enum HHToggle {
	BarToggle,
	FastToggle,
}
impl Distribution<HHToggle> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HHToggle {
		match rng.gen_range(0..=1) {
			0 => HHToggle::BarToggle,
			1 => HHToggle::FastToggle,
			_ => unreachable!(),
		}
	}
}

#[derive(Copy, Clone, Default)]
pub struct Drop0 {
	hh: HH,
	skipped: bool,
	ch_prev: bool,
	oh_prev: bool,
	hh_toggle: Option<HHToggle>,
	ch_toggle: bool,
	oh_toggle: bool,
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
			log_send(
				conn,
				&control_change(channel_id, cc_parameter(CC_LAYER, 0), 0),
			);
			self.ch_toggle = false;
			self.oh_toggle = false;
			self.hh_toggle = None;
			if transition == Transition::No {
				if !self.ch_prev && ch {
					self.hh_toggle = Some(rng.gen());
					self.ch_toggle = true;
					if !self.oh_prev && oh {
						self.oh_toggle = true;
					}
				} else if !self.oh_prev && oh {
					self.hh_toggle = Some(HHToggle::FastToggle);
					self.oh_toggle = true;
				} else if (!oh && self.oh_prev) || (!ch && self.ch_prev) {
					self.hh_toggle = Some(HHToggle::BarToggle);
					self.oh_toggle = !oh && self.oh_prev;
					self.ch_toggle = !ch && self.ch_prev;
				}
			}
			self.ch_prev = ch;
			self.oh_prev = oh;
		}

		if transition.is_transition_in() {
			if t == 0 {
				log_send(
					conn,
					&control_change(channel_id, cc_parameter(CC_LENGTH, 0), 127),
				);
			}
			if t == 12 {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
			}
		}

		if let Some(h) = self.hh_toggle {
			if h == HHToggle::BarToggle {
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
			} else {
				if t == 0 || t == 24 || t == 48 {
					log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
				}
				if t == 84 {
					log_send(
						conn,
						&control_change(channel_id, cc_parameter(CC_LAYER, 0), 1 << 6),
					);
					log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
				}
			}
		} else {
			if t == 0 || t == 24 || t == 48 {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
			}

			if t == 12 && rng.gen_bool(DOUBLED_PROBA) {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
			}

			if !transition.is_transition_out() {
				if t == 72 {
					if !rng.gen_bool(SKIPPED_PROBA) {
						log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
						self.skipped = false;
					} else {
						self.skipped = true;
					}
				}

				if t == 84 && self.skipped {
					log_send(
						conn,
						&control_change(channel_id, cc_parameter(CC_LAYER, 0), 1 << 6),
					);
					log_send(conn, &start_note(channel_id, SP1, param_value(0.0)));
				}
			}
		}

		if oh ^ self.oh_toggle {
			if t < 72 || !self.oh_toggle {
				self.hh.trigger_oh(step, conn, root, rng);
			}
		}
		if ch ^ self.ch_toggle {
			if t < 72 || !self.ch_toggle {
				self.hh.trigger_ch(step, conn, root);
			}
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
			if transition.is_transition_in() {
				log_send(
					conn,
					&control_change(channel_id, cc_parameter(CC_LENGTH, 0), 127),
				);
			}
			log_send(
				conn,
				&control_change(channel_id, cc_parameter(CC_LAYER, 0), 0),
			);
		}

		let mut no_hh = false;

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
		} else if let Transition::Out(Stage::Break) = transition {
			if t == 0 {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.6)));
			} else if t == 24 {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.7)));
			} else if t == 48 {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.8)));
			} else if t == 72 {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.9)));
			}
		} else {
			if t == 0 || t == 24 || t == 48 {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.6)));
			} else if t == 12 && rng.gen_bool(DOUBLED_PROBA) {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.6)));
			} else if t == 72 && !rng.gen_bool(SKIPPED_PROBA) {
				log_send(conn, &start_note(channel_id, SP1, param_value(0.6)));
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
