use crate::acid::Acid;
use crate::state::LeadState;
use crate::trig::Trig;
use crate::{LEAD0_CHANNEL, LEAD1_CHANNEL};
use midir::MidiOutputConnection;

pub struct Lead1 {
    acid: Acid,
    state: LeadState,
    prev_state: LeadState,
    end_note: bool,
    start_note: bool,
    prev_psy_note: u8,
}

impl Lead1 {
    pub fn new(acid: Acid) -> Self {
        Self {
            acid,
            state: LeadState::default(),
            prev_state: LeadState::default(),
            end_note: false,
            start_note: false,
            prev_psy_note: 0,
        }
    }

    //TODO: we can omit sending end_note on acid to create tension
    pub fn get_trig(&mut self, step: u32, root: u8) -> Vec<Trig> {
        let mut res = vec![];
        if self.end_note {
            self.end_note = false;
            match self.prev_state {
                LeadState::Acid => {
                    let prev_note = self.acid.get_prev_note();
                    res.push(Trig {
                        start_end: false,
                        channel_id: LEAD1_CHANNEL,
                        note: prev_note.0,
                        velocity: prev_note.1,
                    });
                }
                LeadState::Psy => {
                    res.push(Trig {
                        start_end: false,
                        channel_id: LEAD1_CHANNEL,
                        note: self.prev_psy_note,
                        velocity: 100,
                    });
                }
                _ => {}
            }
        }
        if self.start_note {
            self.start_note = false;
            self.prev_psy_note = root;
            res.push(Trig {
                start_end: false,
                channel_id: LEAD1_CHANNEL,
                note: root,
                velocity: 100,
            });
        }
        match self.state {
            LeadState::Acid => res.append(&mut self.acid.get_trig(step, root)),
            _ => {}
        }
        res
    }

    //TODO add rng for acid maybe
    pub fn toggle(&mut self, state: &LeadState) {
        match self.state {
            LeadState::Acid | LeadState::Psy => self.end_note = true,
            _ => {}
        }
        if let LeadState::Psy = state {
            self.start_note = true;
        }

        if let LeadState::Acid = state {
            self.acid.next_pattern();
        }

        self.prev_state = self.state;
        self.state = *state;
    }

    pub fn get_state(&self) -> LeadState {
        self.state
    }
}
