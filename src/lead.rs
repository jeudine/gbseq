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
    next_state: Option<Lead0State>,
    wait: bool,
    end_note: bool,
    prev_atm_note: u8,
}

pub struct Lead1 {
    acid: Acid,
    state: Lead1State,
    prev_state: Lead1State,
    next_state: Option<Lead1State>,
    wait: bool,
    end_note: bool,
    prev_psy_note: u8,
}

impl Lead0 {
    pub fn new(arp: Arp) -> Self {
        Self {
            arp,
            state: Lead0State::default(),
            prev_state: Lead0State::default(),
            next_state: None,
            end_note: false,
            prev_atm_note: 0,
            wait: false,
        }
    }

    pub fn get_state(&self) -> (Lead0State, Option<String>) {
        if let Lead0State::Arp = self.state {
            (Lead0State::Arp, Some(self.arp.get_name()))
        } else {
            (self.state, None)
        }
    }

    pub fn get_trigs(&mut self, step: u32, root: u8) -> Vec<Trig> {
        let mut start_note = false;
        if step % 96 == 0 {
            if let Some(s) = self.next_state {
                if self.wait {
                    self.wait = false;
                    start_note = true;
                    match self.state {
                        Lead0State::Arp | Lead0State::Atm => self.end_note = true,
                        _ => {}
                    }
                    self.state = s;
                    self.next_state = None;
                } else {
                    self.wait = true;
                }
            }
        }

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
            Lead0State::Arp => {
                res.append(&mut self.arp.get_trig(step, root));
            }
            Lead0State::Atm => {
                if step % 96 == 0 && start_note {
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
            _ => {}
        }
        res
    }

    pub fn toggle(&mut self, state: Lead0State, scale: Scale) {
        if let Lead0State::Arp = state {
            self.arp.next_pattern(scale);
        }

        self.prev_state = self.state;
        self.next_state = Some(state);
    }

    pub fn on(&self) -> bool {
        match self.state {
            Lead0State::Arp => true,
            Lead0State::Atm => true,
            _ => match self.next_state {
                Some(_) => true,
                None => false,
            },
        }
    }

    pub fn toogle_arp(&mut self) {
        if let Lead0State::Arp = self.state {
            self.arp.toggle_sub();
        }
    }
}
impl Lead1 {
    pub fn new(acid: Acid) -> Self {
        Self {
            acid,
            state: Lead1State::default(),
            prev_state: Lead1State::default(),
            next_state: None,
            wait: false,
            end_note: false,
            prev_psy_note: 0,
        }
    }

    pub fn get_trigs(&mut self, step: u32, root: u8) -> Vec<Trig> {
        let mut start_note = false;
        if step % 96 == 0 {
            if let Some(s) = self.next_state {
                if self.wait {
                    self.wait = false;
                    start_note = true;
                    match self.state {
                        Lead1State::Acid | Lead1State::Psy => self.end_note = true,
                        _ => {}
                    }
                    self.state = s;
                    self.next_state = None;
                } else {
                    self.wait = true;
                }
            }
        }

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

        match self.state {
            Lead1State::Acid => {
                res.append(&mut self.acid.get_trig(step, root));
            }
            Lead1State::Psy => {
                if step % 96 == 0 && start_note {
                    self.prev_psy_note = root;
                    res.push(Trig {
                        start_end: true,
                        channel_id: LEAD1_CHANNEL,
                        note: root,
                        velocity: 100,
                    });
                }
            }
            _ => {}
        }
        res
    }

    pub fn toggle(&mut self, state: Lead1State, scale: Scale) {
        if let Lead1State::Acid = state {
            self.acid.next_pattern(scale);
        }

        self.prev_state = self.state;
        self.next_state = Some(state);
    }

    pub fn get_state(&self) -> (Lead1State, Option<String>) {
        if let Lead1State::Acid = self.state {
            (Lead1State::Acid, Some(self.acid.get_name()))
        } else {
            (self.state, None)
        }
    }

    pub fn on(&self) -> bool {
        match self.state {
            Lead1State::Acid => true,
            Lead1State::Psy => true,
            _ => match self.next_state {
                Some(_) => true,
                None => false,
            },
        }
    }
}
