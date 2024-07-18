use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;
use tseq::sequence::{
    cc_parameter, control_change, param_value, start_note, Sequence, CC_LAYER, SP1,
};
use tseq::{log_send, trigger, Stage, StateData, Transition, PERC_CHANNEL};

const DOUBLED_PROBA: f64 = 0.33;

#[derive(Clone, Copy, Default)]
pub struct Breakbeat0 {}

impl Sequence for Breakbeat0 {
    fn run(
        &mut self,
        step: u32,
        conn: &mut MidiOutputConnection,
        rng: &mut ThreadRng,
        state_data: StateData,
    ) {
        let t = step % 96;

        let mut no_hh = false;

        let transition = state_data.transition;
        if t == 0 && transition.is_transition_in() {
            log_send(
                conn,
                &control_change(PERC_CHANNEL, cc_parameter(CC_LAYER, 0), 0x60),
            );
        }

        // Kicks
        if t == 0 || t == 36 {
            if let Transition::Out(Stage::Drop) = transition {
            } else {
                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
            }
        }

        if t == 12 || t == 48 {
            if rng.gen_bool(DOUBLED_PROBA) {
                if let Transition::Out(Stage::Drop) = transition {
                } else {
                    log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
                }
            }
        }

        if let Transition::Out(Stage::Drop) = transition {
            if t == 0 || t == 12 || t == 36 || t == 48 || t == 60 {
                log_send(
                    conn,
                    &start_note(PERC_CHANNEL, SP1, param_value(t as f32 / 192.0)),
                );
            } else if t == 84 {
                log_send(
                    conn,
                    &control_change(PERC_CHANNEL, cc_parameter(CC_LAYER, 0), 1 << 6),
                );

                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
            }

            if t >= 72 {
                no_hh = true;
            }
        }
        if !no_hh {
            trigger(conn, &state_data.hh);
            trigger(conn, &state_data.perc);
        }
        trigger(conn, &state_data.lead0);
        trigger(conn, &state_data.lead1);
    }
}
