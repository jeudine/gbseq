use crate::pattern::Note;

pub enum Timing {
	Note,
	Tie,
	Rest,
}

pub struct AcidTrig {
	note: (Note, u8),
	vel: u8,
	slide: bool,
	timing: Timing,
}
