use tseq::run;

fn main() {
	match run() {
		Ok(_) => {}
		Err(e) => {
			eprintln!("[ERROR] {e}");
			std::process::exit(1);
		}
	}
}
