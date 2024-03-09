use midir::MidiOutputConnection;
use tseq::sequence::Sequence;

#[derive(Copy, Clone)]
pub struct Break0 {}

impl Sequence for Break0 {
	fn run(&self, step: u32, conn: &mut MidiOutputConnection, channel_id: u8) {}
}
