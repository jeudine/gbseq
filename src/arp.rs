use crate::scale::Scale;
use crate::trig::Trig;
pub struct Arp {}

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
