use crate::acid::Acid;
use crate::log_send;
use crate::sequence::end_note;
use crate::state::SelLead;
use midir::MidiOutputConnection;

pub const LEAD_CHANNEL: u8 = 3;

#[derive(Default)]
pub struct Lead {
	pub acid: Acid,
	state: SelLead,
	end_note: bool,
}

impl Lead {
	pub fn run(&mut self, step: u32, conn: &mut MidiOutputConnection, root: u8) {
		if self.end_note {
			self.end_note = false;
			log_send(conn, &end_note(LEAD_CHANNEL, root, 100));
		}
		match self.state {
			SelLead::Acid => self.acid.trigger(step, conn, root),
			SelLead::Psy => { /*TODO*/ }
			SelLead::None => {}
		}
	}

	pub fn switch(&mut self, state: &SelLead) {
		match self.state {
			SelLead::Acid | SelLead::Psy => self.end_note = true,
			_ => {}
		}
		self.state = *state;
	}
}
