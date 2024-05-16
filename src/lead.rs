use crate::acid::Acid;
use midir::MidiOutputConnection;
use tseq::sequence::Lead;

pub struct LeadSel {
	acid: Acid,
}

impl LeadSel {
	fn play_lead(&self, lead: Lead, step: u32, conn: &mut MidiOutputConnection, root: u8) {
		match lead {
			Lead::Acid => self.acid.trigger(step, conn, root),
			Lead::None => {}
		}
	}
}
