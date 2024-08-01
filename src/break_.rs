use gbseq::{
    cc_parameter, control_change, log_send, only_trigger_ch, only_trigger_oh, param_value,
    start_note, trigger, Sequence, Stage, StateData, Transition, CC_LAYER, PERC_CHANNEL, SP1,
};
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;

#[derive(Copy, Clone, Default)]
pub struct Break0 {}

impl Sequence for Break0 {
    fn run(
        &mut self,
        step: u32,
        conn: &mut MidiOutputConnection,
        _rng: &mut ThreadRng,
        state_data: StateData,
    ) {
        let t = step % 96;

        let mut no_hh = false;

        let transition = state_data.transition;

        if t == 0 && transition.is_transition_in() {
            log_send(
                conn,
                &control_change(PERC_CHANNEL, cc_parameter(CC_LAYER, 0), 0),
            );
        }
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
                    &control_change(PERC_CHANNEL, cc_parameter(CC_LAYER, 0), 26),
                );

                log_send(conn, &start_note(PERC_CHANNEL, SP1, param_value(0.0)));
            }

            if t >= 72 {
                no_hh = true;
            }
        } else if let Transition::Out(Stage::Breakbeat) = transition {
            if t >= 72 {
                no_hh = true;
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
