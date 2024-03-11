use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::default::Default;
use std::f32::consts::PI;
use std::ops::Range;

#[derive(Debug, Default, Clone, Copy)]
pub struct LFO {
	pub speed: f32,
	pub depth: f32,
}

impl LFO {
	pub fn get_val(&self, step: u32) -> f32 {
		self.depth * f32::sin(self.speed * 2.0 * PI * step as f32 / 96.0)
	}

	pub fn set_rng(&mut self, speeds: &[f32], depth_range: Range<f32>, rng: &mut ThreadRng) {
		self.speed = *speeds.choose(rng).unwrap();
		self.depth = rng.gen_range(depth_range);
	}
}

pub trait Sequence {
	fn run(
		&mut self,
		step: u32,
		conn: &mut MidiOutputConnection,
		channel_id: u8,
		rng: &mut ThreadRng,
		oh: bool,
		ch: bool,
		oh_lfo: &LFO,
		ch_lfo: &LFO,
	);
}

pub const SP1: u8 = 48;
pub const SP2: u8 = 49;
pub const SP3: u8 = 50;
pub const SP4: u8 = 51;
pub const NOTE_ON: u8 = 0x90;
pub const NOTE_OFF: u8 = 0x80;

pub fn start_note(channel_id: u8, note: u8, velocity: u8) -> Vec<u8> {
	vec![NOTE_ON | channel_id, note, velocity]
}

pub fn end_note(channel_id: u8, note: u8, velocity: u8) -> Vec<u8> {
	vec![NOTE_OFF | channel_id, note, velocity]
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
