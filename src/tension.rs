use gbseq::{
    cc_parameter, control_change, log_send, only_trigger_ch, only_trigger_oh, param_value,
    start_note, trigger, Sequence, Stage, Stage::*, StateData, Transition, CC_LAYER, CC_LEVEL,
    RAMPLE_CHANNEL, SP1,
};
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;

const SKIP_PROBA: f64 = 0.2;
const DOUBLE_PROBA: f64 = 0.2;

#[derive(Copy, Clone, Default)]
enum State {
    #[default]
    HighPass,
    Rumble,
}

impl State {
    fn is_high_pass(&self) -> bool {
        match self {
            State::HighPass => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Tension0 {
    state: State,
}

impl Sequence for Tension0 {
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
            match transition {
                Transition::In(s) => match s {
                    Break | Breakbeat => {
                        self.state = State::Rumble;
                        log_send(
                            conn,
                            &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 78),
                        );
                    }
                    _ => {
                        self.state = State::HighPass;
                        log_send(
                            conn,
                            &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LEVEL, 0), 90),
                        );
                        log_send(
                            conn,
                            &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 0),
                        );
                    }
                },
                _ => {}
            }
        }
        let mut no_hh = false;

        if transition.is_transition_in() {
            match self.state {
                State::Rumble => {
                    if t == 0 {
                        log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.5)));
                    } else if t == 24 {
                        log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.4)));
                    } else if t == 48 {
                        log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.3)));
                    } else if t == 72 {
                        log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.2)));
                    }
                }

                State::HighPass => {
                    if t == 0 || t == 24 || t == 48 || t == 72 {
                        log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
                    }
                }
            }
        } else if let Transition::Out(Stage::Drop) = transition {
            self.t_out_0(conn, &mut no_hh, t);
        } else if let Transition::Out(Stage::Breakbeat) = transition {
            self.t_out_0(conn, &mut no_hh, t);
        } else if let Transition::Out(Stage::Break) = transition {
            if t == 0 {
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.6)));
            } else if t == 24 {
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.7)));
            } else if t == 48 {
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.8)));
            } else if t == 72 {
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.9)));
            } else if t == 84 {
                log_send(
                    conn,
                    &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LEVEL, 0), 63),
                );
            }
        } else {
            if t == 0 || t == 24 || t == 48 {
                self.send(conn);
            } else if t == 12 && self.state.is_high_pass() && rng.gen_bool(DOUBLE_PROBA) {
                self.send(conn);
            } else if t == 72 && (!rng.gen_bool(SKIP_PROBA) || !self.state.is_high_pass()) {
                self.send(conn);
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

impl Tension0 {
    fn send(&self, conn: &mut MidiOutputConnection) {
        match self.state {
            State::Rumble => {
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
            }
            _ => {
                log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.6)));
            }
        }
    }

    fn t_out_0(&self, conn: &mut MidiOutputConnection, no_hh: &mut bool, t: u32) {
        match self.state {
            State::Rumble => {
                if t == 0 {
                    log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.3)));
                } else if t == 24 {
                    log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.4)));
                } else if t == 48 {
                    log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.5)));
                } else if t == 84 {
                    log_send(
                        conn,
                        &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 26),
                    );
                    log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
                }
            }
            State::HighPass => {
                if t == 0 {
                    log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.6)));
                } else if t == 24 {
                    log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.5)));
                } else if t == 48 {
                    log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.4)));
                } else if t == 84 {
                    log_send(
                        conn,
                        &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LAYER, 0), 26),
                    );
                    log_send(
                        conn,
                        &control_change(RAMPLE_CHANNEL, cc_parameter(CC_LEVEL, 0), 63),
                    );

                    log_send(conn, &start_note(RAMPLE_CHANNEL, SP1, param_value(0.0)));
                }
            }
        }

        if t >= 72 {
            *no_hh = true;
        }
    }
}
