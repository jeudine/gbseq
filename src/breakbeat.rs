use gbseq::{
    cc_parameter, control_change, log_send, only_trigger_ch, only_trigger_oh, param_value,
    start_note, trigger, Sequence, Stage, StateData, Transition, CC_LAYER, PERC_CHANNEL, SP1,
};
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;

const DOUBLE_PROBA: f64 = 0.33;
const SKIP_PROBA: f64 = 0.166;

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
        let t = step % 36;

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
            if t == 84 {
                log_send(
                    conn,
                    &control_change(PERC_CHANNEL, cc_parameter(CC_LAYER, 0), 1 << 6),
                );

                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
            }

            if t >= 72 {
                no_hh = true;
            }
        } else if transition.is_transition_out() && t == 95 {
        } else {
            if t == 0 {
                if rng.gen_bool(SKIP_PROBA) {
                    self.skipped = true;
                } else {
                    log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
                    self.skipped = false;
                }
            }

            if t == 12 && (self.skipped || rng.gen_bool(DOUBLE_PROBA)) {
                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
            }
        }

        if !no_hh {
            if state_data.ch_on {
                only_trigger_ch(&state_data.hh, conn);
            }
            if state_data.oh_on {
                only_trigger_oh(&state_data.hh, conn);
            }
            if state_data.perc_on {
                trigger(conn, &state_data.perc);
            }
        }
        trigger(conn, &state_data.lead0);
        trigger(conn, &state_data.lead1);
    }
}
