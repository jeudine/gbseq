mod break_;
mod breakbeat;
mod drop;
mod high_pass;
use break_::Break0;
use breakbeat::Breakbeat0;
use drop::Drop0;
use high_pass::HighPass0;
use std::env;
use tseq::sequence::Sequence;
use tseq::{
    run, Acid, AcidLead, Arp, ArpDiv::*, ArpLead, Note, Pattern, Perc, Rythm, Scale::*, Timing::*,
};

fn main() {
    let break0 = Break0::default();
    let highpass0 = HighPass0::default();
    let drop0 = Drop0::default();
    let breakbeat0 = Breakbeat0::default();

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

    // Percs
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

    // Arp
    let arp0 = ArpLead::new(
        vec![
            vec![(0, 0), (1, 0), (5, 0), (7, 0)],
            vec![(0, 0), (1, 0), (5, 0), (8, 0)],
        ],
        T8,
        vec![PhrygianMode],
        "0",
    );
    let arp1 = ArpLead::new(
        vec![vec![(7, 0), (8, 0), (11, 0), (0, 1), (11, 0), (8, 0)]],
        T8,
        vec![HarmonicMinor],
        "1",
    );
    let arp2 = ArpLead::new(
        vec![
            vec![(0, 0), (3, 0), (7, 0), (3, 0)],
            vec![(0, 0), (2, 0), (7, 0), (2, 0)],
        ],
        T8,
        vec![NaturalMinor, HarmonicMinor],
        "2",
    );

    let arp = Arp::new(vec![arp0, arp1, arp2]);

    // Acid
    let acid_0 = AcidLead::new(
        vec![
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((11, 0), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((8, 0), 127, true, Note),
            ((0, 0), 89, false, Tie),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
        ],
        vec![HarmonicMinor],
        "0",
    );

    let acid_1 = AcidLead::new(
        vec![
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 1), 127, false, Note),
            ((0, 0), 89, true, Note),
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((11, 0), 127, true, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((8, 0), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((11, 0), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 1), 127, true, Note),
            ((0, 0), 89, false, Tie),
            ((0, 0), 89, false, Note),
        ],
        vec![HarmonicMinor],
        "1",
    );

    let acid_2 = AcidLead::new(
        vec![
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, -1), 89, false, Note),
            ((0, 1), 127, true, Note),
            ((0, 0), 89, false, Tie),
            ((0, 0), 89, false, Rest),
        ],
        vec![NaturalMinor, HarmonicMinor, PhrygianMode],
        "2",
    );

    let acid_3 = AcidLead::new(
        vec![
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((8, 0), 89, false, Note),
            ((11, 0), 89, false, Note),
            ((12, 0), 127, false, Note),
            ((11, 0), 89, false, Note),
            ((8, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
        ],
        vec![HarmonicMinor],
        "3",
    );
    let acid_4 = AcidLead::new(
        vec![
            ((0, 1), 89, true, Note),
            ((0, 1), 89, false, Tie),
            ((3, 1), 89, false, Note),
            ((3, 1), 89, false, Tie),
            ((8, 1), 127, false, Note),
            ((8, 1), 89, false, Tie),
            ((8, 1), 89, false, Tie),
            ((8, 1), 89, false, Tie),
            ((0, 1), 89, true, Note),
            ((0, 1), 89, false, Tie),
            ((2, 1), 89, false, Note),
            ((2, 1), 89, false, Tie),
            ((8, 1), 127, false, Note),
            ((8, 1), 89, false, Tie),
            ((8, 1), 89, false, Tie),
            ((8, 1), 89, false, Tie),
            ((10, 0), 89, true, Note),
            ((10, 0), 89, false, Tie),
            ((2, 1), 89, false, Note),
            ((2, 1), 89, false, Tie),
            ((7, 1), 127, false, Note),
            ((7, 1), 89, false, Tie),
            ((7, 1), 89, false, Tie),
            ((7, 1), 89, false, Tie),
            ((8, 0), 89, true, Note),
            ((8, 0), 89, false, Tie),
            ((0, 1), 89, false, Note),
            ((0, 1), 89, false, Tie),
            ((5, 1), 127, false, Note),
            ((5, 1), 89, false, Tie),
            ((5, 1), 89, false, Tie),
            ((5, 1), 89, false, Tie),
            ((10, 0), 89, true, Note),
            ((10, 0), 89, false, Tie),
            ((2, 1), 89, false, Note),
            ((2, 1), 89, false, Tie),
            ((7, 1), 127, false, Note),
            ((7, 1), 89, false, Tie),
            ((7, 1), 89, false, Tie),
            ((7, 1), 89, false, Tie),
            ((10, 0), 89, true, Note),
            ((10, 0), 89, false, Tie),
            ((0, 1), 89, false, Note),
            ((0, 1), 89, false, Tie),
            ((5, 1), 127, false, Note),
            ((5, 1), 89, false, Tie),
            ((5, 1), 89, false, Tie),
            ((5, 1), 89, false, Tie),
            ((7, 0), 89, true, Note),
            ((7, 0), 89, false, Tie),
            ((10, 0), 89, false, Note),
            ((10, 0), 89, false, Tie),
            ((3, 1), 127, false, Note),
            ((3, 1), 89, false, Tie),
            ((3, 1), 89, false, Tie),
            ((3, 1), 89, false, Tie),
            ((7, 0), 89, true, Note),
            ((7, 0), 89, false, Tie),
            ((8, 0), 89, false, Note),
            ((8, 0), 89, false, Tie),
            ((10, 0), 127, false, Note),
            ((10, 0), 89, false, Tie),
            ((10, 0), 89, false, Tie),
            ((10, 0), 89, false, Tie),
        ],
        vec![NaturalMinor],
        "I Follow you",
    );

    let acid = Acid::new(vec![acid_0, acid_1, acid_2, acid_3, acid_4]);

    let args: Vec<String> = env::args().collect();

    let port = if args.len() == 2 {
        Some(args[1].parse::<u32>().unwrap())
    } else {
        None
    };

    //TODO: print at the begining all the MIDI channels used
    //TODO: print which acid and which arps (+infos nb subpattern arps) are played
    match run(1, patterns, perc, arp, acid, port) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("[ERROR] {}", e);
            std::process::exit(1);
        }
    }
}
