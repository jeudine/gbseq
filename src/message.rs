use crate::state::State;
use crate::Channel;
use std::sync::{Arc, Condvar, Mutex};

pub const CLOCK: u8 = 0xf8;
pub const START: u8 = 0xfa;
pub const CONTINUE: u8 = 0xfb;
pub const STOP: u8 = 0xfc;
pub const NOTE_ON: u8 = 0x90;
pub const NOTE_OFF: u8 = 0x80;

pub fn messages_gen(channel_arc: &Arc<(Mutex<Channel>, Condvar)>, state_arc: &Arc<Mutex<State>>) {
	let (channel, cvar) = &**channel_arc;
	let mut channel = channel.lock().unwrap();
	loop {
		channel = cvar.wait(channel).unwrap();

		let state = state_arc.lock().unwrap();

		if !state.running {
			continue;
		}

		if channel.step == 95 {
			let _ = channel.conn.send(&[START]);
		}

		if channel.step % 24 == 0 {
			let _ = channel.conn.send(&start_note(0, 60, 80));
		}

		if channel.step % 24 == 12 {
			let _ = channel.conn.send(&start_note(0, 61, 80));
		}

		if channel.step % 24 == 18 {
			let _ = channel.conn.send(&end_note(0, 61, 80));
		}

		if channel.step % 24 == 6 {
			let _ = channel.conn.send(&end_note(0, 60, 80));
		}
	}
}

pub fn start_note(channel: u8, note: u8, velocity: u8) -> Vec<u8> {
	vec![NOTE_ON | channel, note, velocity]
}

pub fn end_note(channel: u8, note: u8, velocity: u8) -> Vec<u8> {
	vec![NOTE_OFF | channel, note, velocity]
}

pub fn param_value(v: f32) -> u8 {
	if v < -1.0 {
		return 0;
	}
	if v > 1.0 {
		return 127;
	}
	63 + (v * 63.0).round() as u8
}
