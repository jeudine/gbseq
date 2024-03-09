mod drop;
mod mbreak;
use drop::Drop0;
use mbreak::Break0;
use tseq::sequence::Sequence;
use tseq::{run, Pattern};

fn main() {
	let break0 = Break0 {};
	let drop0 = Drop0 {};

	let patterns = [155, 156, 158, 160, 162, 164, 166, 168]
		.iter()
		.map(|bpm| {
			let s_break: Vec<Box<dyn Sequence + Send>> = vec![Box::new(break0)];
			let s_drop: Vec<Box<dyn Sequence + Send>> = vec![Box::new(drop0)];
			let s_high_pass: Vec<Box<dyn Sequence + Send>> = vec![Box::new(break0)];
			let s_breakbeat: Vec<Box<dyn Sequence + Send>> = vec![Box::new(break0)];
			let break_to_drop: Vec<(Box<dyn Sequence + Send>, u32)> = vec![(Box::new(break0), 0)];
			let drop_to_break: Vec<(Box<dyn Sequence + Send>, u32)> = vec![(Box::new(break0), 0)];
			Pattern {
				bpm: *bpm as u8,
				s_break,
				s_drop,
				s_high_pass,
				s_breakbeat,
				break_to_drop,
				drop_to_break,
			}
		})
		.collect();

	match run(1, patterns) {
		Ok(_) => {}
		Err(e) => {
			eprintln!("[ERROR] {e}");
			std::process::exit(1);
		}
	}
}
