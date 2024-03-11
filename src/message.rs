use crate::log_send;
use crate::sequence::Sequence;
use crate::state::State;
use crate::Channel;
use std::sync::{Arc, Condvar, Mutex};

pub const CLOCK: u8 = 0xf8;
pub const START: u8 = 0xfa;
pub const CONTINUE: u8 = 0xfb;
pub const STOP: u8 = 0xfc;

pub fn messages_gen(
	channel_arc: &Arc<(Mutex<Channel>, Condvar)>,
	state_arc: &Arc<Mutex<State>>,
	channel_id: u8,
) {
	let (channel, cvar) = &**channel_arc;
	let mut channel = channel.lock().unwrap();
	let mut rng = rand::thread_rng();
	loop {
		channel = cvar.wait(channel).unwrap();

		let mut state = state_arc.lock().unwrap();

		if !state.running {
			continue;
		}

		if channel.step == 95 {
			log_send(&mut channel.conn, &[START]);
		}

		let oh = state.oh;
		let ch = state.ch;
		let oh_lfo = state.oh_lfo;
		let ch_lfo = state.ch_lfo;

		let sequence: &mut Box<dyn Sequence + Send> =
			state.get_cur_sequence(channel.step, &mut rng);
		sequence.run(
			channel.step,
			&mut channel.conn,
			channel_id,
			&mut rng,
			oh,
			ch,
			&oh_lfo,
			&ch_lfo,
		);
	}
}
