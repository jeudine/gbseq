use crate::message;
use crate::Channel;
use std::mem::drop;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub fn clock_gen(channel_arc: &Arc<(Mutex<Channel>, Condvar)>) {
	loop {
		let (channel, cvar) = &**channel_arc;
		let mut channel = channel.lock().unwrap();
		let _ = channel.conn.send(&[message::CLOCK]);
		let period = channel.period_us;
		channel.step += 1;
		// Unlock the mutex before the sleep
		drop(channel);
		cvar.notify_one();
		sleep(Duration::from_micros(period));
	}
}

pub fn compute_period_us(bpm: u8) -> u64 {
	60 * 1000000 / 24 / bpm as u64
}
