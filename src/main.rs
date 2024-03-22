mod breakbeat;
mod drop;
mod hh;
mod mbreak;
use breakbeat::Breakbeat0;
use drop::{Drop0, HighPass0};
use mbreak::Break0;
use tseq::sequence::Sequence;
use tseq::Note;
use tseq::{run, Pattern};

fn main() {
	let break0 = Break0::default();
	let highpass0 = HighPass0::default();
	let drop0 = Drop0::default();
	let breakbeat0 = Breakbeat0::default();

	//let patterns = [155, 156, 158, 160, 162, 164, 166, 168]
	let patterns = [(155, Note::C), (156, Note::F)]
		.iter()
		.map(|(bpm, root)| {
			let s_break: Vec<Box<dyn Sequence + Send>> = vec![Box::new(break0)];
			let s_drop: Vec<Box<dyn Sequence + Send>> = vec![Box::new(drop0)];
			let s_high_pass: Vec<Box<dyn Sequence + Send>> = vec![Box::new(highpass0)];
			let s_breakbeat: Vec<Box<dyn Sequence + Send>> = vec![Box::new(breakbeat0)];
			Pattern {
				bpm: *bpm as u8,
				s_break,
				s_drop,
				s_high_pass,
				s_breakbeat,
				root: *root,
			}
		})
		.collect();

	match run(1, patterns) {
		Ok(_) => {}
		Err(e) => {
			eprintln!("[ERROR] {}", e);
			std::process::exit(1);
		}
	}
}
