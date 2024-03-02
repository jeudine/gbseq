use tseq::{run, Pattern};

fn main() {
	let pattern = Pattern { bpm: vec![155] };
	match run(1, pattern) {
		Ok(_) => {}
		Err(e) => {
			eprintln!("[ERROR] {e}");
			std::process::exit(1);
		}
	}
}
