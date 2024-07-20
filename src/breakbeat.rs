use gbseq::{
    cc_parameter, control_change, log_send, param_value, start_note, trigger, Sequence, Stage,
    StateData, Transition, CC_BITS, CC_LAYER, PERC_CHANNEL, SP1,
};
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;

const DOUBLED_PROBA: f64 = 0.33;
const BITS_PROBA: f64 = 0.33;

#[derive(Clone, Copy, Default)]
pub struct Breakbeat0 {
    bits_set: bool,
}

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

        if let Transition::Out(Stage::Drop) = transition {
            if t == 0 || t == 12 || t == 36 || t == 48 || t == 60 {
                log_send(
                    conn,
                    &control_change(
                        PERC_CHANNEL,
                        cc_parameter(CC_BITS, 0),
                        param_value(t as f32 / 96.0),
                    ),
                );

                log_send(
                    conn,
                    &start_note(PERC_CHANNEL, SP1, param_value(t as f32 / 192.0)),
                );
            } else if t == 84 {
                log_send(
                    conn,
                    &control_change(PERC_CHANNEL, cc_parameter(CC_LAYER, 0), 1 << 6),
                );
                log_send(
                    conn,
                    &control_change(PERC_CHANNEL, cc_parameter(CC_BITS, 0), param_value(0.0)),
                );

                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
            }

            if t >= 72 {
                no_hh = true;
            }
        } else {
            if t == 0 || t == 36 {
                if rng.gen_bool(BITS_PROBA) {
                    log_send(
                        conn,
                        &control_change(PERC_CHANNEL, cc_parameter(CC_BITS, 0), param_value(0.5)),
                    );
                    self.bits_set = true;
                } else if self.bits_set {
                    self.bits_set = false;
                    log_send(
                        conn,
                        &control_change(PERC_CHANNEL, cc_parameter(CC_BITS, 0), 63),
                    );
                }
                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
            }

            if t == 12 || t == 48 {
                if rng.gen_bool(DOUBLED_PROBA) {
                    log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
                }
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
