use gbseq::{
    cc_parameter, control_change, log_send, only_trigger_ch, only_trigger_oh, param_value,
    start_note, trigger, Sequence, Stage, StateData, Transition, CC_LAYER, RAMPLE_CHANNEL, SP1,
};
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;

const DOUBLE_PROBA: f64 = 0.25;
const SKIP_PROBA: f64 = 0.25;

#[derive(Clone, Copy, Default)]
pub struct Breakbeat0 {
    skipped: bool,
}

impl Sequence for Breakbeat0 {
    fn run(
        &mut self,
        step: u32,
        conn: &mut MidiOutputConnection,
        rng: &mut ThreadRng,
        state_data: StateData,
    ) {
        let mut no_hh = false;

        let transition = state_data.transition;

        if transition.is_transition_in() {
            let t = step % 96;
            if t == 0 {
                log_send(
                    conn,
                    &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 52),
                );
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
            } else if t == 12 {
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
            } else if t == 24 {
                log_send(
                    conn,
                    &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 104),
                );
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
            } else if t == 72 {
                log_send(
                    conn,
                    &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 52),
                );
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
            }
        } else if transition.is_transition_out() {
            let t = step % 96;
            if t == 24 {
                log_send(
                    conn,
                    &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 104),
                );
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
            }

            if let Transition::Out(Stage::Drop) = transition {
                if t >= 72 {
                    no_hh = true;
                } else if t == 84 {
                    log_send(
                        conn,
                        &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 26),
                    );
                    log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
                }
            }
        } else {
            let t = step % 36;
            if t == 0 && !self.skipped {
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
            } else if t == 12 && !self.skipped && rng.gen_bool(DOUBLE_PROBA) {
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
            } else if t == 24 {
                if self.skipped {
                    self.skipped = false;
                    log_send(
                        conn,
                        &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 56),
                    );
                } else if rng.gen_bool(SKIP_PROBA) {
                    self.skipped = true;
                    log_send(
                        conn,
                        &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 104),
                    );
                    log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
                }
            }
        }

        if !no_hh {
            if state_data.ch_on {
                only_trigger_ch(&state_data.hh, conn);
            }
            if state_data.oh_on {
                only_trigger_oh(&state_data.hh, conn);
            }
        }
        trigger(conn, &state_data.stab);
        trigger(conn, &state_data.lead0);
        trigger(conn, &state_data.lead1);
    }
}
