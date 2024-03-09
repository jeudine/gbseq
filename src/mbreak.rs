use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use tseq::sequence::Sequence;

#[derive(Copy, Clone)]
pub struct Break0 {}

impl Sequence for Break0 {
	fn run(
		&mut self,
		step: u32,
		conn: &mut MidiOutputConnection,
		channel_id: u8,
		rng: &mut ThreadRng,
	) {
	}
}
