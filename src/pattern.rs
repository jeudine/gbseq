use crate::sequence::Sequence;
use crate::state::Stage;

pub struct Pattern {
    pub bpm: u8,
    pub root: Note,
    pub s_break: Vec<Box<dyn Sequence + Send>>,
    pub s_drop: Vec<Box<dyn Sequence + Send>>,
    pub s_tension: Vec<Box<dyn Sequence + Send>>,
    pub s_breakbeat: Vec<Box<dyn Sequence + Send>>,
}

impl Pattern {
    pub fn get_sequence(&mut self, seq_id: usize, stage: Stage) -> &mut Box<dyn Sequence + Send> {
        match stage {
            Stage::Break => &mut self.s_break[seq_id],
            Stage::Drop => &mut self.s_drop[seq_id],
            Stage::Tension => &mut self.s_tension[seq_id],
            Stage::Breakbeat => &mut self.s_breakbeat[seq_id],
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Note {
    #[default]
    C,
    CS,
    D,
    DS,
    E,
    F,
    FS,
    G,
    GS,
    A,
    AS,
    B,
}

impl Note {
    pub fn get_midi(&self) -> u8 {
        match self {
            Note::C => 60,
            Note::CS => 61,
            Note::D => 62,
            Note::DS => 63,
            Note::E => 64,
            Note::F => 65,
            Note::FS => 54,
            Note::G => 55,
            Note::GS => 56,
            Note::A => 57,
            Note::AS => 58,
            Note::B => 59,
        }
    }

    pub fn get_str(&self) -> &str {
        match self {
            Note::C => "C",
            Note::CS => "C#",
            Note::D => "D",
            Note::DS => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::FS => "F#",
            Note::G => "G",
            Note::GS => "G#",
            Note::A => "A",
            Note::AS => "A#",
            Note::B => "B",
        }
    }
}
