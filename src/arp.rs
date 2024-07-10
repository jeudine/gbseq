use crate::scale::Scale;
use crate::trig::Trig;

pub enum ArpDiv {
    T4,
    T8,
    T16,
}

#[derive(Default, Clone)]
pub struct ArpLead {
    pattern: Vec<Vec<(u8, i8)>>,
    scales: Vec<Scale>,
    played: bool,
}

pub struct Arp {
    patterns: Vec<ArpLead>,
    cur_id: usize,
    prev_note: u8,
}

impl ArpLead {}

impl Arp {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_trig(&mut self, step: u32, root: u8) -> Vec<Trig> {
        todo!()
    }

    pub fn get_prev_note(&self) -> (u8, u8) {
        todo!()
    }

    pub fn next_pattern(&mut self, scale: Scale) {
        todo!()
    }
}
