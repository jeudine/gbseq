use crate::message;
use crate::state::State;
use crate::Channel;
use std::default::Default;
use std::mem::drop;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::sleep;
use std::time::Duration;

enum System {
	StartStop,
	Quit,
}

enum Stage {
	Drop,
	Break,
	Breakbeat,
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
					let _ = channel.conn.send(&[message::STOP]);
				}
				return false;
			}
			System::Quit => return true,
		}
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
				_ => {}
			}
		}
		action
	}
}
