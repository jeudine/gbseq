use crate::acid::Acid;
use crate::arp::Arp;
use crate::hh::HH;
use crate::lead::{Lead0, Lead1};
use crate::pattern::{Note, Pattern};
use crate::perc::Perc;
use crate::scale::Scale;
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
pub enum Lead0State {
    #[default]
    None,
    Arp,
    Atm,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Lead1State {
    #[default]
    None,
    Acid,
    Psy,
}

impl fmt::Display for Lead0State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lead0State::None => write!(f, "None"),
            Lead0State::Arp => write!(f, "Arp"),
            Lead0State::Atm => write!(f, "Atm"),
        }
    }
}

impl fmt::Display for Lead1State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lead1State::None => write!(f, "None"),
            Lead1State::Acid => write!(f, "Acid"),
            Lead1State::Psy => write!(f, "Psy"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SelPatt {
    Prev,
    Next,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SelScale {
    _Prev,
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
    pub sel_lead0: Option<Lead0State>,
    pub sel_lead1: Option<Lead1State>,
    pub sel_scale: Option<SelScale>,
    pub stage: Stage,
    next_stage: Stage,
    pub cur_seq_id: usize,
    hh: HH,
    perc: Perc,
    lead0: Lead0,
    lead1: Lead1,
    pub ch_toggle: bool,
    pub oh_toggle: bool,
    pub perc_toggle: bool,
    transition: Transition,
    scale: Scale,
    pub arp_toggle: bool,
}

pub struct Info {
    pub root: Note,
    pub bpm: u8,
    pub lead0: (Lead0State, Option<String>),
    pub lead1: (Lead1State, Option<String>),
    pub scale: Scale,
}

pub struct StateData {
    pub transition: Transition,
    pub root_note: u8,
    pub hh: Vec<Trig>,
    pub perc: Vec<Trig>,
    pub lead0: Vec<Trig>,
    pub lead1: Vec<Trig>,

    pub ch_on: bool,
    pub oh_on: bool,
    pub perc_on: bool,
    pub lead0_on: bool,
    pub lead1_on: bool,
}

impl State {
    pub fn new(patterns: Vec<Pattern>, perc: Perc, arp: Arp, acid: Acid) -> Self {
        Self {
            running: false,
            patterns,
            perc,
            lead1: Lead1::new(acid),
            lead0: Lead0::new(arp),
            cur_pattern_id: 0,
            sel_patt: None,
            sel_lead0: None,
            sel_lead1: None,
            sel_scale: None,
            stage: Stage::default(),
            next_stage: Stage::default(),
            cur_seq_id: 0,
            hh: HH::default(),
            oh_toggle: false,
            ch_toggle: false,
            perc_toggle: false,
            transition: Transition::default(),
            scale: Scale::default(),
            arp_toggle: false,
        }
    }

    pub fn get_sequence(&mut self) -> &mut Box<dyn Sequence + Send> {
        self.patterns[self.cur_pattern_id].get_sequence(self.cur_seq_id, self.stage)
    }

    pub fn update(&mut self, step: u32, rng: &mut ThreadRng) -> (StateData, Option<(SelPatt, u8)>) {
        let mut sel_patt: Option<(SelPatt, u8)> = None;
        if step % 96 == 0 {
            self.hh.start_bar();
            if self.next_stage != self.stage
                && (self.transition == Transition::No || self.transition.is_transition_in())
            {
                self.transition = Transition::Out(self.next_stage);
            } else if self.transition.is_transition_out() {
                self.transition = Transition::In(self.stage);
                self.stage = self.next_stage;
                self.toggle(rng);
            } else if self.next_stage == self.stage && self.transition.is_transition_in() {
                self.transition = Transition::No;
            } else if self.next_stage == self.stage && self.transition == Transition::No {
                self.toggle(rng);
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

            if let Some(s) = self.sel_scale {
                self.scale = match s {
                    SelScale::_Prev => self.scale.prev(),
                    SelScale::Next => self.scale.next(),
                }
            }
        }

        let root_note = self.get_cur_root();

        (
            StateData {
                transition: self.transition,
                root_note,
                hh: self.hh.get_trigs(step, root_note, rng),
                perc: self.perc.get_trigs(step),
                lead0: self.lead0.get_trigs(step, root_note),
                lead1: self.lead1.get_trigs(step, root_note),
                ch_on: self.hh.ch_on(),
                oh_on: self.hh.oh_on(),
                lead0_on: self.lead0.on(),
                lead1_on: self.lead1.on(),
                perc_on: self.perc.on(),
            },
            sel_patt,
        )
    }

    pub fn toggle(&mut self, rng: &mut ThreadRng) {
        if self.ch_toggle {
            self.hh.toggle_ch();
            self.ch_toggle = false;
        }
        if self.oh_toggle {
            self.hh.toggle_oh();
            self.oh_toggle = false;
        }
        if self.perc_toggle {
            self.perc.toggle(rng);
            self.perc_toggle = false;
        }
        if let Some(l) = self.sel_lead0 {
            self.lead0.toggle(l, self.scale);
            self.sel_lead0 = None;
        }
        if let Some(l) = self.sel_lead1 {
            self.lead1.toggle(l, self.scale);
            self.sel_lead1 = None;
        }

        if self.arp_toggle {
            self.lead0.toogle_arp();
            self.arp_toggle = false;
        }
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

    pub fn get_info(&self) -> Info {
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

        let lead0 = if let Some(l) = self.sel_lead0 {
            (l, None)
        } else {
            self.lead0.get_state()
        };

        let lead1 = if let Some(l) = self.sel_lead1 {
            (l, None)
        } else {
            self.lead1.get_state()
        };

        let scale = if let Some(s) = self.sel_scale {
            match s {
                SelScale::_Prev => self.scale.prev(),
                SelScale::Next => self.scale.next(),
            }
        } else {
            self.scale
        };

        Info {
            root: self.patterns[i].root,
            bpm: self.patterns[i].bpm,
            lead0,
            lead1,
            scale,
        }
    }
}
