use crate::sequence::{end_note, start_note};
use crate::trig::Trig;
use crate::{log_send, LEAD1_CHANNEL};

#[derive(Default, Clone, Copy)]
pub enum Timing {
    Note,
    Tie,
    #[default]
    Rest,
}

use Timing::*;

#[derive(Default)]
pub struct AcidTrig {
    note: (u8, i8),
    vel: u8,
    slide: bool,
    timing: Timing,
}

#[derive(Default)]
pub struct Acid {
    patterns: Vec<Vec<AcidTrig>>,
    cur_id: usize,
    prev_note: (Timing, u8, u8),
}

impl Acid {
    pub fn new() -> Self {
        let pattern_0 = Self::new_pattern(vec![
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((11, 0), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((8, 0), 127, true, Note),
            ((0, 0), 89, false, Tie),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
        ]);

        let pattern_1 = Self::new_pattern(vec![
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 1), 127, false, Note),
            ((0, 0), 89, true, Note),
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((11, 0), 127, true, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((8, 0), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((11, 0), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 1), 127, true, Note),
            ((0, 0), 89, false, Tie),
            ((0, 0), 89, false, Note),
        ]);

        let pattern_2 = Self::new_pattern(vec![
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, -1), 89, false, Note),
            ((0, 1), 127, true, Note),
            ((0, 0), 89, false, Tie),
            ((0, 0), 89, false, Rest),
        ]);
        let pattern_3 = Self::new_pattern(vec![
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((8, 0), 89, false, Note),
            ((11, 0), 89, false, Note),
            ((12, 0), 127, false, Note),
            ((11, 0), 89, false, Note),
            ((8, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 1), 127, false, Note),
            ((0, 0), 89, false, Note),
            ((0, 0), 89, false, Note),
        ]);

        Self {
            patterns: vec![pattern_0, pattern_2, pattern_1, pattern_3],
            cur_id: 0,
            prev_note: (Rest, 0, 0),
        }
    }

    pub fn get_trig(&mut self, step: u32, root: u8) -> Vec<Trig> {
        let mut res = vec![];
        if step % 6 == 0 {
            let t = step / 6;
            let cur_trig = t as usize % self.patterns[self.cur_id].len();
            self.prev_note.0 = self.patterns[self.cur_id][cur_trig].timing;
            let cur_note = &self.patterns[self.cur_id][cur_trig];

            let no_end = if let Tie = cur_note.timing {
                true
            } else {
                false
            };

            let prev_note = self.prev_note;

            let note = root + cur_note.note.0 + (cur_note.note.1 * 12) as u8;
            match cur_note.timing {
                Note => {
                    res.push(Trig {
                        start_end: true,
                        channel_id: LEAD1_CHANNEL,
                        note,
                        velocity: cur_note.vel,
                    });
                    self.prev_note.1 = note;
                    self.prev_note.2 = cur_note.vel;
                }
                _ => {}
            }

            // When there is a slide end the note after the previous one started
            if !no_end && cur_note.slide {
                match prev_note.0 {
                    Note | Tie => res.push(Trig {
                        start_end: false,
                        channel_id: LEAD1_CHANNEL,
                        note: prev_note.1,
                        velocity: prev_note.2,
                    }),
                    _ => {}
                }
            }
        } else if step % 6 == 3 {
            let t = (step + 3) / 6;
            let cur_trig = t as usize % self.patterns[self.cur_id].len();
            self.prev_note.0 = self.patterns[self.cur_id][cur_trig].timing;
            let cur_note = &self.patterns[self.cur_id][cur_trig];

            let no_end = if let Tie = cur_note.timing {
                true
            } else {
                false
            };

            if !no_end && !cur_note.slide {
                match self.prev_note.0 {
                    Note | Tie => res.push(Trig {
                        start_end: false,
                        channel_id: LEAD1_CHANNEL,
                        note: self.prev_note.1,
                        velocity: self.prev_note.2,
                    }),

                    _ => {}
                }
            }
        }
        res
    }

    pub fn next_pattern(&mut self) {
        let len = self.patterns.len();
        self.cur_id = (self.cur_id + 1) % len;
    }

    pub fn new_pattern(x: Vec<((u8, i8), u8, bool, Timing)>) -> Vec<AcidTrig> {
        x.iter()
            .map(|u| AcidTrig {
                note: u.0,
                vel: u.1,
                slide: u.2,
                timing: u.3,
            })
            .collect()
    }

    pub fn get_prev_note(&self) -> (u8, u8) {
        (self.prev_note.1, self.prev_note.2)
    }
}
