use crate::acid::Acid;
use crate::log_send;
use crate::sequence::{end_note, start_note};
use crate::state::LeadState;
use midir::MidiOutputConnection;

pub const LEAD_CHANNEL: u8 = 3;

#[derive(Default)]
pub struct Lead {
    pub acid: Acid,
    state: LeadState,
    prev_state: LeadState,
    end_note: bool,
    start_note: bool,
}

impl Lead {
    pub fn run(&mut self, step: u32, conn: &mut MidiOutputConnection, root: u8) {
        if self.end_note {
            self.end_note = false;
            match self.prev_state {
                LeadState::Acid => {
                    let prev_note = self.acid.get_prev_note();
                    log_send(conn, &end_note(LEAD_CHANNEL, prev_note.0, prev_note.1));
                }
                LeadState::Psy => {
                    log_send(conn, &end_note(LEAD_CHANNEL, root, 100));
                }
                _ => {}
            }
        }
        if self.start_note {
            self.start_note = false;
            log_send(conn, &start_note(LEAD_CHANNEL, root, 100));
        }
        match self.state {
            LeadState::Acid => self.acid.trigger(step, conn, root),
            _ => {}
        }
    }

    pub fn switch(&mut self, state: &LeadState) {
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
