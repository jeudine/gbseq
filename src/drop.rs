use gbseq::{
    cc_parameter, control_change, log_send, only_trigger_ch, only_trigger_oh, param_value,
    start_note, trigger, Sequence, StateData, Transition, CC_LAYER, CC_LEVEL, RAMPLE_CHANNEL, SP1,
};
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

const SKIPPED_PROBA: f64 = 0.2;
const DOUBLED_PROBA: f64 = 0.2;

#[derive(Copy, Clone, PartialEq)]
enum Toggle {
    BarToggle,
    MidToggle,
    FastToggle,
}
impl Distribution<Toggle> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Toggle {
        match rng.gen_range(0..=1) {
            0 => Toggle::BarToggle,
            1 => Toggle::MidToggle,
            2 => Toggle::FastToggle,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Drop0 {
    skipped: bool,
    ch_prev: bool,
    oh_prev: bool,
    stab_prev: bool,
    toggle: Option<Toggle>,
    ch_toggle: bool,
    oh_toggle: bool,
    lead0_prev: bool,
    lead1_prev: bool,
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
        let lead0 = state_data.lead0_on;
        let lead1 = state_data.lead1_on;
        let stab = state_data.stab_on;

        if t == 0 {
            log_send(
                conn,
                &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 0),
            );
            self.ch_toggle = false;
            self.oh_toggle = false;
            self.toggle = None;
            if transition == Transition::No {
                self.ch_toggle = self.ch_prev ^ ch;
                self.oh_toggle = self.oh_prev ^ oh;
                let lead0_toggle = !self.lead0_prev && lead0;
                let lead1_toggle = !self.lead1_prev && lead1;

                if (!oh && self.oh_prev) || (!ch && self.ch_prev) {
                    self.toggle = Some(Toggle::BarToggle);
                } else if self.ch_toggle || self.oh_toggle || lead0_toggle || lead1_toggle {
                    self.toggle = Some(rng.gen());
                }
            }
            self.ch_prev = ch;
            self.oh_prev = oh;
            self.stab_prev = stab;
            self.lead0_prev = lead0;
            self.lead1_prev = lead1;
        }

        if transition.is_transition_in() {
            if t == 12 {
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
            }
        }

        if let Some(h) = self.toggle {
            match h {
                Toggle::BarToggle => Drop0::bar_toggle(t, conn),
                Toggle::MidToggle => Drop0::mid_toggle(t, conn),
                Toggle::FastToggle => Drop0::fast_toggle(t, conn),
            }
        } else {
            if t == 0 || t == 24 || t == 48 {
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
            }

            if t == 12 && !transition.is_transition_in() && rng.gen_bool(DOUBLED_PROBA) {
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
            }

            if !transition.is_transition_out() {
                if t == 72 {
                    if !rng.gen_bool(SKIPPED_PROBA) {
                        log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
                        self.skipped = false;
                    } else {
                        self.skipped = true;
                    }
                }

                if t == 84 && self.skipped {
                    log_send(
                        conn,
                        &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 26),
                    );
                    log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
                }
            } else {
                if t == 72 {
                    log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
                }
            }
        }

        // We want the HH to be off when we toggle on and on before 72 when we toggle off
        if (oh ^ self.oh_toggle) && (t < 72 || !self.oh_toggle) {
            only_trigger_oh(&state_data.hh, conn);
        }
        if (ch ^ self.ch_toggle) && (t < 72 || !self.ch_toggle) {
            only_trigger_ch(&state_data.hh, conn);
        }
        trigger(conn, &state_data.stab);
        trigger(conn, &state_data.lead0);
        trigger(conn, &state_data.lead1);
    }
}

impl Drop0 {
    fn bar_toggle(t: u32, conn: &mut MidiOutputConnection) {
        if t == 0 {
            log_send(
                conn,
                &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LEVEL, 0), 90),
            );
            log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.6)));
        } else if t == 24 {
            log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.5)));
        } else if t == 48 {
            log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.4)));
        } else if t == 84 {
            log_send(
                conn,
                &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LEVEL, 0), 63),
            );
            log_send(
                conn,
                &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 26),
            );
            log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
        }
    }

    fn fast_toggle(t: u32, conn: &mut MidiOutputConnection) {
        if t == 0 || t == 24 || t == 48 {
            log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
        } else if t == 84 {
            log_send(
                conn,
                &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 26),
            );
            log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
        }
    }

    fn mid_toggle(t: u32, conn: &mut MidiOutputConnection) {
        if t == 0 {
            log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
        } else if t == 24 {
            log_send(
                conn,
                &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LEVEL, 0), 90),
            );
            log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.5)));
        } else if t == 48 {
            log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.4)));
        } else if t == 84 {
            log_send(
                conn,
                &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LEVEL, 0), 63),
            );
            log_send(
                conn,
                &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 26),
            );
            log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
        }
    }
}
