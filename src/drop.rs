use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use tseq::sequence::{
    cc_parameter, control_change, param_value, start_note, Sequence, CC_LAYER, CC_LENGTH, CC_LEVEL,
    SP1,
};
use tseq::{log_send, only_trigger_ch, only_trigger_oh, StateData, Transition, PERC_CHANNEL};

const SKIPPED_PROBA: f64 = 0.2;
const DOUBLED_PROBA: f64 = 0.2;

#[derive(Copy, Clone, PartialEq)]
enum HHToggle {
    BarToggle,
    FastToggle,
}
impl Distribution<HHToggle> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HHToggle {
        match rng.gen_range(0..=1) {
            0 => HHToggle::BarToggle,
            1 => HHToggle::FastToggle,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Drop0 {
    skipped: bool,
    ch_prev: bool,
    oh_prev: bool,
    hh_toggle: Option<HHToggle>,
    ch_toggle: bool,
    oh_toggle: bool,
}

impl Sequence for Drop0 {
    fn run(
        &mut self,
        step: u32,
        conn: &mut MidiOutputConnection,
        rng: &mut ThreadRng,
        state_data: StateData,
    ) {
        let t = step % 96;

        let transition = state_data.transition;
        let ch = state_data.ch_on;
        let oh = state_data.oh_on;

        // TODO: add half bar transition
        if t == 0 {
            log_send(
                conn,
                &control_change(PERC_CHANNEL, cc_parameter(CC_LAYER, 0), 0),
            );
            self.ch_toggle = false;
            self.oh_toggle = false;
            self.hh_toggle = None;
            if transition == Transition::No {
                if !self.ch_prev && ch {
                    self.hh_toggle = Some(rng.gen());
                    self.ch_toggle = true;
                    if !self.oh_prev && oh {
                        self.oh_toggle = true;
                    }
                } else if !self.oh_prev && oh {
                    self.hh_toggle = Some(HHToggle::FastToggle);
                    self.oh_toggle = true;
                } else if (!oh && self.oh_prev) || (!ch && self.ch_prev) {
                    self.hh_toggle = Some(HHToggle::BarToggle);
                    self.oh_toggle = !oh && self.oh_prev;
                    self.ch_toggle = !ch && self.ch_prev;
                }
            }
            self.ch_prev = ch;
            self.oh_prev = oh;
        }

        if transition.is_transition_in() {
            if t == 0 {
                log_send(
                    conn,
                    &control_change(PERC_CHANNEL, cc_parameter(CC_LENGTH, 0), 127),
                );
                log_send(
                    conn,
                    &control_change(PERC_CHANNEL, cc_parameter(CC_LEVEL, 0), 63),
                );
            }
            if t == 12 {
                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
            }
        }

        if let Some(h) = self.hh_toggle {
            if h == HHToggle::BarToggle {
                Drop0::bar_toggle(t, conn);
            } else {
                Drop0::fast_toggle(t, conn);
            }
        } else {
            if t == 0 || t == 24 || t == 48 {
                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
            }

            if t == 12 && !transition.is_transition_in() && rng.gen_bool(DOUBLED_PROBA) {
                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
            }

            if !transition.is_transition_out() {
                if t == 72 {
                    if !rng.gen_bool(SKIPPED_PROBA) {
                        log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
                        self.skipped = false;
                    } else {
                        self.skipped = true;
                    }
                }

                if t == 84 && self.skipped {
                    log_send(
                        conn,
                        &control_change(PERC_CHANNEL, cc_parameter(CC_LAYER, 0), 1 << 6),
                    );
                    log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
                }
            } else {
                if t == 72 {
                    log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
                }
            }
        }

        if (oh ^ self.oh_toggle) && (t < 72 || !self.oh_toggle) {
            only_trigger_oh(&state_data.hh, conn);
        }
        if (ch ^ self.ch_toggle) && (t < 72 || !self.ch_toggle) {
            only_trigger_ch(&state_data.hh, conn);
        }
    }
}

impl Drop0 {
    fn bar_toggle(t: u32, conn: &mut MidiOutputConnection) {
        if t == 0 {
            log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.6)));
        } else if t == 24 {
            log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.5)));
        } else if t == 48 {
            log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.4)));
        } else if t == 84 {
            log_send(
                conn,
                &control_change(PERC_CHANNEL, cc_parameter(CC_LAYER, 0), 1 << 6),
            );
            log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
        }
    }

    fn fast_toggle(t: u32, conn: &mut MidiOutputConnection) {
        if t == 0 || t == 24 || t == 48 {
            log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
        }
        if t == 84 {
            log_send(
                conn,
                &control_change(PERC_CHANNEL, cc_parameter(CC_LAYER, 0), 1 << 6),
            );
            log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
        }
    }
}
