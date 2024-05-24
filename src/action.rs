use crate::pattern::Note;
use crate::state::{LeadState, SelPatt, Stage, State};
use crate::Channel;
use crate::{log_send, message};
use std::default::Default;
use std::sync::{Arc, Condvar, Mutex};

enum System {
	StartStop,
	Quit,
}

#[derive(Default)]
struct Action {
	system: Option<System>,
	stage: Option<Stage>,
	ch_toggle: bool,
	oh_toggle: bool,
	pattern: Option<SelPatt>,
	lead: Option<LeadState>,
}

pub fn handle(
	s: &String,
	channel_arc: &Arc<(Mutex<Channel>, Condvar)>,
	state_arc: &Arc<Mutex<State>>,
) -> Option<(Note, u8, LeadState)> {
	let action = Action::parse(s);
	let (channel, _) = &**channel_arc;
	let mut state = state_arc.lock().unwrap();

	if let Some(a) = action.system {
		let mut channel = channel.lock().unwrap();
		match a {
			System::StartStop => {
				state.running = !state.running;
				if state.running {
					channel.step = 94;
				} else {
					log_send(&mut channel.conn, &[message::STOP]);
				}
				return Some(state.get_root_note_bpm_lead());
			}
			System::Quit => {
				log_send(&mut channel.conn, &[message::STOP]);
				return None;
			}
		}
	}

	if let Some(a) = action.stage {
		state.set_next_stage(&a);
	}

	state.oh_toggle = action.oh_toggle;
	state.ch_toggle = action.ch_toggle;
	state.sel_patt = action.pattern;
	state.sel_lead = action.lead;

	Some(state.get_root_note_bpm_lead())
}

impl Action {
	fn parse(s: &String) -> Self {
		let mut action = Self::default();
		for c in s.chars() {
			match c {
				's' => action.system = Some(System::StartStop),
				'q' => action.system = Some(System::Quit),
				'1' => action.stage = Some(Stage::Break),
				'2' => action.stage = Some(Stage::Drop),
				'3' => action.stage = Some(Stage::HighPass),
				'4' => action.stage = Some(Stage::Breakbeat),
				'5' => action.ch_toggle = true,
				'6' => action.oh_toggle = true,
				'7' => action.pattern = Some(SelPatt::Prev),
				'8' => action.pattern = Some(SelPatt::Next),
				'/' => action.lead = Some(LeadState::None),
				'*' => action.lead = Some(LeadState::Acid),
				'-' => action.lead = Some(LeadState::Psy),
				_ => {}
			}
		}
		action
	}
}
