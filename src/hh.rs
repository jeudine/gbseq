use crate::trig::{trigger_single, Trig};
use crate::{CH_CHANNEL, OH_CHANNEL};
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;

const DOUBLED_PROBA: f64 = 0.2;

#[derive(Copy, Clone, Default)]
pub struct HH {
    off_step_ch: u32,
    off_step_oh: u32,
    ch_active: bool,
    oh_active: bool,
    last_oh_bar: bool,
    last_ch_bar: bool,
}

impl HH {
    pub fn get_trigs(&mut self, step: u32, root: u8, rng: &mut ThreadRng) -> Vec<Trig> {
        let mut res = vec![];
        if self.ch_active || self.last_ch_bar {
            if let Some(t) = self.get_trigs_ch(step, root) {
                res.push(t)
            }
        }
        if self.oh_active || self.last_oh_bar {
            if let Some(t) = self.get_trigs_oh(step, root, rng) {
                res.push(t)
            }
        }
        res
    }

    fn get_trigs_ch(&mut self, step: u32, root: u8) -> Option<Trig> {
        if step % 6 == 0 {
            self.off_step_ch = step + 3;
            return Some(Trig {
                start_end: true,
                channel_id: CH_CHANNEL,
                note: root,
                velocity: 100,
            });
        }

        if step == self.off_step_ch {
            return Some(Trig {
                start_end: false,
                channel_id: CH_CHANNEL,
                note: root,
                velocity: 100,
            });
        }
        None
    }

    //TODO: try different OH patterns
    fn get_trigs_oh(&mut self, step: u32, root: u8, rng: &mut ThreadRng) -> Option<Trig> {
        if step % 24 == 12 || (step % 96 == 72 && rng.gen_bool(DOUBLED_PROBA)) {
            self.off_step_oh = step + 6;
            return Some(Trig {
                start_end: true,
                channel_id: OH_CHANNEL,
                note: root,
                velocity: 100,
            });
        }
        if step == self.off_step_oh {
            return Some(Trig {
                start_end: false,
                channel_id: OH_CHANNEL,
                note: root,
                velocity: 100,
            });
        }
        None
    }

    pub fn toggle_oh(&mut self) {
        self.oh_active = !self.oh_active;
        if !self.oh_active {
            self.last_oh_bar = true;
        }
    }

    // To make the hh stay one bar longer after we turn them off
    pub fn start_bar(&mut self) {
        self.last_oh_bar = false;
        self.last_ch_bar = false;
    }

    pub fn toggle_ch(&mut self) {
        self.ch_active = !self.ch_active;
        if !self.ch_active {
            self.last_ch_bar = true;
        }
    }

    pub fn ch_on(&self) -> bool {
        self.ch_active
    }

    pub fn oh_on(&self) -> bool {
        self.oh_active
    }
}

pub fn only_trigger_ch(trigs: &Vec<Trig>, conn: &mut MidiOutputConnection) {
    for t in trigs {
        if t.channel_id == CH_CHANNEL {
            trigger_single(conn, t)
        }
    }
}
pub fn only_trigger_oh(trigs: &Vec<Trig>, conn: &mut MidiOutputConnection) {
    for t in trigs {
        if t.channel_id == OH_CHANNEL {
            trigger_single(conn, t)
        }
    }
}
