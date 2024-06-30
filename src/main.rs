mod break_;
mod breakbeat;
mod drop;
mod high_pass;
use break_::Break0;
use breakbeat::Breakbeat0;
use drop::Drop0;
use high_pass::HighPass0;
use tseq::sequence::Sequence;
use tseq::Note;
use tseq::{run, Pattern, Perc, Rythm};

fn main() {
    let break0 = Break0::default();
    let highpass0 = HighPass0::default();
    let drop0 = Drop0::default();
    let mut breakbeat0 = Breakbeat0::default();
    /*
        breakbeat0.push_rythm([3, 2, 1]);
        breakbeat0.push_rythm([4, 3, 1]);
        breakbeat0.push_rythm([5, 3, 2]);
    */

    let patterns = [
        (155, Note::C),
        (156, Note::A),
        (158, Note::C),
        (160, Note::A),
        (162, Note::C),
        (180, Note::A),
    ]
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

    let er_1 = Rythm::compute_euclidean_rythm(1);
    let er_2 = Rythm::compute_euclidean_rythm(2);
    let er_3 = Rythm::compute_euclidean_rythm(3);
    let er_4 = Rythm::compute_euclidean_rythm(4);
    let er_5 = Rythm::compute_euclidean_rythm(5);

    let perc = Perc::new(vec![
        [er_3, er_2, er_1],
        [er_4, er_3, er_1],
        [er_5, er_3, er_2],
    ]);

    //TODO: maybe hardcode and print at the begining all the MIDI channels used
    match run(1, patterns, perc) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("[ERROR] {}", e);
            std::process::exit(1);
        }
    }
}
