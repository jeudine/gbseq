use crate::acid::Acid;
use crate::lead::Lead;
use crate::pattern::{Note, Pattern};
use crate::perc::Perc;
use crate::sequence::Sequence;
use crate::trig::Trig;
use rand::rngs::ThreadRng;
use std::default::Default;
use std::fmt;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Stage {
    #[default]
    Break,
    Drop,
    HighPass,
    Breakbeat,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Transition {
    #[default]
    No,
    In(Stage),
    Out(Stage),
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum LeadState {
    #[default]
    None,
    Acid,
    Psy,
}

impl fmt::Display for LeadState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LeadState::None => write!(f, "None"),
            LeadState::Acid => write!(f, "Acid"),
            LeadState::Psy => write!(f, "Psy"),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SelPatt {
    Prev,
    Next,
}

impl Transition {
    pub fn is_transition_in(&self) -> bool {
        if let Transition::In(_) = self {
            return true;
        }
        false
    }
    pub fn is_transition_out(&self) -> bool {
        if let Transition::Out(_) = self {
            return true;
        }
        false
    }
}

pub struct State {
    pub running: bool,
    pub patterns: Vec<Pattern>,
    pub cur_pattern_id: usize,
    pub sel_patt: Option<SelPatt>,
    pub sel_lead: Option<LeadState>,
    pub stage: Stage,
    next_stage: Stage,
    pub cur_seq_id: usize,
    oh: bool,
    ch: bool,
    pub oh_toggle: bool,
    pub ch_toggle: bool,
    transition: Transition,
    pub lead: Lead,
    pub perc: Perc,
}

impl State {
    pub fn new(patterns: Vec<Pattern>, perc: Perc) -> Self {
        let acid = Acid::new(); //TODO: pass as a parameter
        Self {
            running: false,
            patterns: patterns,
            cur_pattern_id: 0,
            sel_patt: None,
            sel_lead: None,
            stage: Stage::default(),
            next_stage: Stage::default(),
            cur_seq_id: 0,
            oh: false,
            ch: false,
            oh_toggle: false,
            ch_toggle: false,
            transition: Transition::default(),
            lead: Lead::new(acid),
            perc,
        }
    }

    pub fn get_sequence(&mut self) -> &mut Box<dyn Sequence + Send> {
        self.patterns[self.cur_pattern_id].get_sequence(self.cur_seq_id, self.stage)
    }

    //TODO: update percs
    pub fn update(
        &mut self,
        step: u32,
        rng: &mut ThreadRng,
    ) -> (Transition, bool, bool, Vec<Trig>, u8, Option<(SelPatt, u8)>) {
        let mut sel_patt: Option<(SelPatt, u8)> = None;
        if step % 96 == 0 {
            if self.next_stage != self.stage
                && (self.transition == Transition::No || self.transition.is_transition_in())
            {
                self.transition = Transition::Out(self.next_stage);
            } else if self.transition.is_transition_out() {
                self.transition = Transition::In(self.stage);
                self.stage = self.next_stage;
                if self.oh_toggle {
                    self.oh = !self.oh;
                    self.oh_toggle = false;
                }
                if self.ch_toggle {
                    self.ch = !self.ch;
                    self.ch_toggle = false;
                }
                if let Some(l) = self.sel_lead {
                    self.lead.switch(&l);
                    self.sel_lead = None;
                }
            } else if self.next_stage == self.stage && self.transition.is_transition_in() {
                self.transition = Transition::No;
            } else if self.next_stage == self.stage && self.transition == Transition::No {
                if self.oh_toggle {
                    self.oh = !self.oh;
                    self.oh_toggle = false;
                }
                if self.ch_toggle {
                    self.ch = !self.ch;
                    self.ch_toggle = false;
                }
                if let Some(l) = self.sel_lead {
                    self.lead.switch(&l);
                    self.sel_lead = None;
                }
            }

            if let Some(s) = self.sel_patt {
                if match s {
                    SelPatt::Prev => self.prev_pattern(),
                    SelPatt::Next => self.next_pattern(),
                } {
                    let bpm = self.patterns[self.cur_pattern_id].bpm;
                    sel_patt = Some((s, bpm));
                }
                self.sel_patt = None;
            }
        }
        let root = self.get_cur_root();
        let perc_trigs = self.perc.get_trigs(step);

        (
            self.transition,
            self.ch,
            self.oh,
            perc_trigs,
            root,
            sel_patt,
        )
    }

    pub fn set_next_stage(&mut self, stage: &Stage) {
        self.next_stage = *stage;
    }

    fn prev_pattern(&mut self) -> bool {
        if self.cur_pattern_id > 0 {
            self.cur_pattern_id -= 1;
            return true;
        }
        false
    }

    fn next_pattern(&mut self) -> bool {
        if self.cur_pattern_id < self.patterns.len() - 1 {
            self.cur_pattern_id += 1;
            return true;
        }
        false
    }

    fn get_cur_root(&self) -> u8 {
        self.patterns[self.cur_pattern_id].root.get_midi()
    }

    pub fn get_root_note_bpm_lead(&self) -> (Note, u8, LeadState) {
        let mut i = self.cur_pattern_id;
        if let Some(p) = self.sel_patt {
            match p {
                SelPatt::Prev => {
                    if i > 0 {
                        i -= 1
                    }
                }
                SelPatt::Next => {
                    if i < self.patterns.len() - 1 {
                        i += 1
                    }
                }
            };
        }

        let mut lead = self.lead.get_state();
        if let Some(l) = self.sel_lead {
            lead = l;
        }
        (self.patterns[i].root, self.patterns[i].bpm, lead)
    }
}
