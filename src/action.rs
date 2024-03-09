use crate::state::{Stage, State};
use crate::Channel;
use crate::{log_send, message};
use std::default::Default;
use std::mem::drop;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::sleep;
use std::time::Duration;

enum System {
	StartStop,
	Quit,
}

enum HH {
	OnOff,
}
#[derive(Default)]
struct Action {
	system: Option<System>,
	stage: Option<Stage>,
	oh: Option<HH>,
	ch: Option<HH>,
}

pub fn handle(
	s: &String,
	channel_arc: &Arc<(Mutex<Channel>, Condvar)>,
	state_arc: &Arc<Mutex<State>>,
) -> bool {
	let action = Action::parse(s);
	let (channel, _) = &**channel_arc;
	let mut state = state_arc.lock().unwrap();

	if let Some(a) = action.system {
		match a {
			System::StartStop => {
				state.running = !state.running;
				let mut channel = channel.lock().unwrap();
				if state.running {
					channel.step = 94;
				} else {
					log_send(&mut channel.conn, &[message::STOP]);
				}
				return false;
			}
			System::Quit => return true,
		}
	}

	if let Some(a) = action.stage {
		state.set_next_stage(&a);
	}

	false
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
				_ => {}
			}
		}
		action
	}
}
