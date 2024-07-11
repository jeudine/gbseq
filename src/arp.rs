use crate::scale::Scale;
use crate::trig::Trig;
use rand::seq::SliceRandom;
use rand::thread_rng;

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

impl ArpLead {
    pub fn new(pattern: Vec<Vec<(u8, i8)>>, scales: Vec<Scale>) -> Self {
        Self {
            pattern,
            scales,
            played: false,
        }
    }
}

impl Arp {
    pub fn new(patterns: Vec<ArpLead>) -> Self {
        //Check that we have at least one pattern for each scale
        let mut sc: [bool; 3] = [false; 3];
        for p in &patterns {
            for s in &p.scales {
                match s {
                    Scale::NaturalMinor => sc[0] = true,
                    Scale::HarmonicMinor => sc[1] = true,
                    Scale::PhrygianMode => sc[2] = true,
                }
            }
        }

        for (i, s) in sc.iter().enumerate() {
            if !s {
                match i {
                    0 => panic!("No arp lead for Natural Minor Scale!"),
                    1 => panic!("No arp lead for Harmonic Minor Scale!"),
                    _ => panic!("No arp lead for Phrygian Mode!"),
                }
            }
        }

        let mut patterns = patterns.clone();
        patterns.shuffle(&mut thread_rng());
        Self {
            patterns,
            cur_id: 0,
            prev_note: 0,
        }
    }

    pub fn get_trig(&mut self, step: u32, root: u8) -> Vec<Trig> {
        todo!()
    }

    pub fn get_prev_note(&self) -> (u8, u8) {
        todo!()
    }

    pub fn next_pattern(&mut self, scale: Scale) {
        let len = self.patterns.len();
        let mut next_id = None;
        for i in 0..len {
            let pattern = &mut self.patterns[i];
            if !pattern.played && pattern.scales.contains(&scale) {
                next_id = Some(i);
                break;
            }
        }
        self.cur_id = if let Some(id) = next_id {
            id
        } else {
            let mut next_id = None;
            for i in 0..len {
                let pattern = &mut self.patterns[i];
                pattern.played = false;
                if let None = next_id {
                    if pattern.scales.contains(&scale) {
                        next_id = Some(i);
                    }
                }
            }

            match next_id {
                Some(id) => id,
                None => panic!("No Arp lead in {}", scale),
            }
        };

        let pattern = &mut self.patterns[self.cur_id];
        pattern.played = true;
    }
}
