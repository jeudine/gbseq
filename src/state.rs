use std::default::Default;

#[derive(Default)]
pub enum Stage {
	#[default]
	Break,
	Drop,
	HighPass,
	Breakbeat,
}

#[derive(Default)]
pub struct State {
	pub running: bool,
	pub stage: Stage,
}
