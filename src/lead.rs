use crate::acid::Acid;
use crate::arp::Arp;
use crate::scale::Scale;
use crate::state::{Lead0State, Lead1State};
use crate::trig::Trig;
use crate::{LEAD0_CHANNEL, LEAD1_CHANNEL};

pub struct Lead0 {
    arp: Arp,
    state: Lead0State,
    prev_state: Lead0State,
    end_note: bool,
    prev_atm_note: u8,
    start_note: bool,
    start_note_reg: bool,
}

pub struct Lead1 {
    acid: Acid,
    state: Lead1State,
    prev_state: Lead1State,
    end_note: bool,
    start_note: bool,
    prev_psy_note: u8,
}

impl Lead0 {
    pub fn new(arp: Arp) -> Self {
        Self {
            arp,
            state: Lead0State::default(),
            prev_state: Lead0State::default(),
            end_note: false,
            prev_atm_note: 0,
            start_note: false,
            start_note_reg: false,
        }
    }

    pub fn get_state(&self) -> Lead0State {
        self.state
    }

    pub fn get_trigs(&mut self, step: u32, root: u8) -> Vec<Trig> {
        let mut res = vec![];
        if self.end_note {
            self.end_note = false;
            match self.prev_state {
                Lead0State::Arp => {
                    let prev_note = self.arp.get_prev_note();
                    res.push(Trig {
                        start_end: false,
                        channel_id: LEAD0_CHANNEL,
                        note: prev_note.0,
                        velocity: prev_note.1,
                    });
                }
                Lead0State::Atm => {
                    res.push(Trig {
                        start_end: false,
                        channel_id: LEAD0_CHANNEL,
                        note: self.prev_atm_note,
                        velocity: 100,
                    });
                    res.push(Trig {
                        start_end: false,
                        channel_id: LEAD0_CHANNEL,
                        note: self.prev_atm_note + 3,
                        velocity: 100,
                    });
                    res.push(Trig {
                        start_end: false,
                        channel_id: LEAD0_CHANNEL,
                        note: self.prev_atm_note + 7,
                        velocity: 100,
                    });
                    res.push(Trig {
                        start_end: false,
                        channel_id: LEAD0_CHANNEL,
                        note: self.prev_atm_note + 12,
                        velocity: 100,
                    });
                }
                _ => {}
            }
        }

        match self.state {
            Lead0State::Arp => res.append(&mut self.arp.get_trig(step, root)),
            Lead0State::Atm => {
                if step % 96 == 0 {
                    if self.start_note {
                        self.start_note_reg = true;
                        self.start_note = false;
                    } else if self.start_note_reg {
                        self.start_note_reg = false;
                        self.prev_atm_note = root;
                        res.push(Trig {
                            start_end: true,
                            channel_id: LEAD0_CHANNEL,
                            note: self.prev_atm_note,
                            velocity: 100,
                        });
                        res.push(Trig {
                            start_end: true,
                            channel_id: LEAD0_CHANNEL,
                            note: self.prev_atm_note + 3,
                            velocity: 100,
                        });
                        res.push(Trig {
                            start_end: true,
                            channel_id: LEAD0_CHANNEL,
                            note: self.prev_atm_note + 7,
                            velocity: 100,
                        });
                        res.push(Trig {
                            start_end: true,
                            channel_id: LEAD0_CHANNEL,
                            note: self.prev_atm_note + 12,
                            velocity: 100,
                        });
                    }
                }
            }
            _ => {}
        }
        res
    }

    pub fn toggle(&mut self, state: Lead0State, scale: Scale) {
        match self.state {
            Lead0State::Arp | Lead0State::Atm => self.end_note = true,
            _ => {}
        }

        if let Lead0State::Atm = state {
            self.start_note = true;
        }

        if let Lead0State::Arp = state {
            self.arp.next_pattern(scale);
        }

        self.prev_state = self.state;
        self.state = state;
    }

    pub fn on(&self) -> bool {
        match self.state {
            Lead0State::Arp => true,
            Lead0State::Atm => true,
            _ => false,
        }
    }
}
impl Lead1 {
    pub fn new(acid: Acid) -> Self {
        Self {
            acid,
            state: Lead1State::default(),
            prev_state: Lead1State::default(),
            end_note: false,
            start_note: false,
            prev_psy_note: 0,
        }
    }

    pub fn get_trigs(&mut self, step: u32, root: u8) -> Vec<Trig> {
        let mut res = vec![];
        if self.end_note {
            self.end_note = false;
            match self.prev_state {
                Lead1State::Acid => {
                    let prev_note = self.acid.get_prev_note();
                    res.push(Trig {
                        start_end: false,
                        channel_id: LEAD1_CHANNEL,
                        note: prev_note.0,
                        velocity: prev_note.1,
                    });
                }
                Lead1State::Psy => {
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
                start_end: true,
                channel_id: LEAD1_CHANNEL,
                note: root,
                velocity: 100,
            });
        }
        match self.state {
            Lead1State::Acid => res.append(&mut self.acid.get_trig(step, root)),
            _ => {}
        }
        res
    }

    pub fn toggle(&mut self, state: Lead1State, scale: Scale) {
        match self.state {
            Lead1State::Acid | Lead1State::Psy => self.end_note = true,
            _ => {}
        }
        if let Lead1State::Psy = state {
            self.start_note = true;
        }

        if let Lead1State::Acid = state {
            self.acid.next_pattern(scale);
        }

        self.prev_state = self.state;
        self.state = state;
    }

    pub fn get_state(&self) -> Lead1State {
        self.state
    }

    pub fn on(&self) -> bool {
        match self.state {
            Lead1State::Acid => true,
            Lead1State::Psy => true,
            _ => false,
        }
    }
}
