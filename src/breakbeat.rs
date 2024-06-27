use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;
use tseq::sequence::{
    cc_parameter, control_change, param_value, start_note, Sequence, CC_FREEZE, CC_LAYER,
    CC_LENGTH, CC_LEVEL, SP1, SP2, SP3, SP4,
};
use tseq::{log_send, trigger, Stage, StateData, Transition, PERC_CHANNEL};

const TRIG_PROBA: f64 = 0.7;
const FREEZE_PROBA: f64 = 0.7;

const DOUBLED_PROBA: f64 = 0.33;

const SP_ARRAY: [u8; 3] = [SP2, SP3, SP4];
const LAYER_ARRAY: [u8; 3] = [0x00, 0x40, 0x60];
const NB_TRIGS: usize = 16;

#[derive(Copy, Clone, Default)]
struct Rythm {
    trigs: [bool; NB_TRIGS],
    k: u8,
}

#[derive(Clone, Default)]
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

        let transition = state_data.transition;
        if t == 0 && transition.is_transition_in() {
            log_send(
                conn,
                &control_change(PERC_CHANNEL, cc_parameter(CC_LAYER, 0), LAYER_ARRAY[2]),
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
            if t == 0 {
                log_send(
                    conn,
                    &control_change(PERC_CHANNEL, cc_parameter(CC_LAYER, 0), 0),
                );

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
        }
    }
}
