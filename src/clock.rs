use crate::message;
use crate::Channel;
use std::mem::drop;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub fn clock_gen(channel: &Arc<Mutex<Channel>>) {
	loop {
		let mut channel = channel.lock().unwrap();
		let _ = channel.conn.send(&[message::CLOCK]);
		let period = channel.period_us;
		// Unlock the mutex before the sleep
		drop(channel);
		sleep(Duration::from_micros(period));
	}
}

pub fn compute_period_us(bpm: u8) -> u64 {
	60 * 1000000 / 24 / bpm as u64
}
