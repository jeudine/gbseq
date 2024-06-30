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
use tseq::{
    log_send, only_trigger_ch, only_trigger_oh, trigger, Stage, StateData, Transition, PERC_CHANNEL,
};

const SKIPPED_PROBA: f64 = 0.2;
const DOUBLED_PROBA: f64 = 0.2;

#[derive(Copy, Clone, Default)]
pub struct HighPass0 {}

impl Sequence for HighPass0 {
    fn run(
        &mut self,
        step: u32,
        conn: &mut MidiOutputConnection,
        rng: &mut ThreadRng,
        state_data: StateData,
    ) {
        let t = step % 96;

        let transition = state_data.transition;
        if t == 0 {
            if transition.is_transition_in() {
                log_send(
                    conn,
                    &control_change(PERC_CHANNEL, cc_parameter(CC_LENGTH, 0), 127),
                );
            }
            log_send(
                conn,
                &control_change(PERC_CHANNEL, cc_parameter(CC_LAYER, 0), 0),
            );
            log_send(
                conn,
                &control_change(PERC_CHANNEL, cc_parameter(CC_LEVEL, 0), 90),
            );
        }

        let mut no_hh = false;

        if let Transition::Out(Stage::Drop) = transition {
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
                log_send(
                    conn,
                    &control_change(PERC_CHANNEL, cc_parameter(CC_LEVEL, 0), 63),
                );

                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
            }
            if t >= 72 {
                no_hh = true;
            }
        } else if let Transition::Out(Stage::Breakbeat) = transition {
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
                log_send(
                    conn,
                    &control_change(PERC_CHANNEL, cc_parameter(CC_LEVEL, 0), 63),
                );

                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
            }
            if t >= 72 {
                no_hh = true;
            }
        } else if let Transition::Out(Stage::Break) = transition {
            if t == 0 {
                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.6)));
            } else if t == 24 {
                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.7)));
            } else if t == 48 {
                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.8)));
            } else if t == 72 {
                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.9)));
            } else if t == 84 {
                log_send(
                    conn,
                    &control_change(PERC_CHANNEL, cc_parameter(CC_LEVEL, 0), 63),
                );
            }
        } else {
            if t == 0 || t == 24 || t == 48 {
                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.6)));
            } else if t == 12 && rng.gen_bool(DOUBLED_PROBA) {
                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.6)));
            } else if t == 72 && !rng.gen_bool(SKIPPED_PROBA) {
                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.6)));
            }
        }

        if !no_hh {
            trigger(conn, &state_data.hh);
        }
        trigger(conn, &state_data.perc);
    }
}
