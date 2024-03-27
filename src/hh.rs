use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;
use tseq::log_send;
use tseq::sequence::{end_note, param_value, start_note, Sequence};

const DOUBLED_PROBA: f64 = 0.2;

#[derive(Copy, Clone, Default)]
pub struct HH {
	off_step_ch: u32,
	off_step_oh: u32,
}

pub const CH_CHANNEL: u8 = 1;
pub const OH_CHANNEL: u8 = 2;

impl HH {
	pub fn trigger_ch(&mut self, step: u32, conn: &mut MidiOutputConnection, root: u8) {
		if step % 6 == 0 {
			log_send(conn, &start_note(CH_CHANNEL, root, 100));
			self.off_step_ch = step + 5;
		}

		if step == self.off_step_ch {
			log_send(conn, &end_note(CH_CHANNEL, root, 100));
		}
	}

	pub fn trigger_ch_dnb(&mut self, step: u32, conn: &mut MidiOutputConnection, root: u8) {
		if step % 12 == 0 {
			log_send(conn, &start_note(CH_CHANNEL, root, 100));
			self.off_step_ch = step + 5;
		}

		if step == self.off_step_ch {
			log_send(conn, &end_note(CH_CHANNEL, root, 100));
		}
	}

	pub fn trigger_oh(
		&mut self,
		step: u32,
		conn: &mut MidiOutputConnection,
		root: u8,
		rng: &mut ThreadRng,
	) {
		if step % 24 == 12 || (step % 96 == 72 && rng.gen_bool(DOUBLED_PROBA)) {
			log_send(conn, &start_note(OH_CHANNEL, root, 100));
			self.off_step_oh = step + 6;
		}

		if step == self.off_step_oh {
			log_send(conn, &end_note(OH_CHANNEL, root, 100));
		}
	}

	pub fn trigger_oh_dnb(&mut self, step: u32, conn: &mut MidiOutputConnection, root: u8) {
		if step % 12 == 6 {
			log_send(conn, &start_note(OH_CHANNEL, root, 100));
			self.off_step_oh = step + 5;
		}

		if step == self.off_step_oh {
			log_send(conn, &end_note(OH_CHANNEL, root, 100));
		}
	}
}
