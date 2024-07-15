use crate::state::{Info, Lead0State, Lead1State, SelPatt, SelScale, Stage, State};
use crate::Channel;
use crate::{log_send, message};
use std::default::Default;
use std::sync::{Arc, Condvar, Mutex};

enum System {
    StartStop,
    Quit,
}

#[derive(Default)]
struct Action {
    system: Option<System>,
    stage: Option<Stage>,
    ch_toggle: bool,
    oh_toggle: bool,
    perc_toggle: bool,
    pattern: Option<SelPatt>,
    lead0: Option<Lead0State>,
    lead1: Option<Lead1State>,
    scale: Option<SelScale>,
    arp_toggle: bool,
}

pub fn handle(
    s: &String,
    channel_arc: &Arc<(Mutex<Channel>, Condvar)>,
    state_arc: &Arc<Mutex<State>>,
) -> Option<Info> {
    let action = Action::parse(s);
    let (channel, _) = &**channel_arc;
    let mut state = state_arc.lock().unwrap();

    if let Some(a) = action.system {
        let mut channel = channel.lock().unwrap();
        match a {
            System::StartStop => {
                state.running = !state.running;
                if state.running {
                    channel.step = 94;
                } else {
                    log_send(&mut channel.conn, &[message::STOP]);
                }
                return Some(state.get_info());
            }
            System::Quit => {
                log_send(&mut channel.conn, &[message::STOP]);
                return None;
            }
        }
    }

    if let Some(a) = action.stage {
        state.set_next_stage(&a);
    }

    state.oh_toggle = action.oh_toggle;
    state.ch_toggle = action.ch_toggle;
    state.perc_toggle = action.perc_toggle;
    state.sel_patt = action.pattern;
    state.sel_lead0 = action.lead0;
    state.sel_lead1 = action.lead1;
    state.arp_toggle = action.arp_toggle;
    state.sel_scale = action.scale;

    Some(state.get_info())
}

impl Action {
    fn parse(s: &String) -> Self {
        let mut action = Self::default();
        let mut opt = false;
        for c in s.chars() {
            match c {
                's' | '=' => action.system = Some(System::StartStop),
                'q' => action.system = Some(System::Quit),
                '0' => action.stage = Some(Stage::Break),
                '1' => action.stage = Some(Stage::Drop),
                '2' => action.stage = Some(Stage::HighPass),
                '3' => action.stage = Some(Stage::Breakbeat),
                '4' => action.ch_toggle = true,
                '5' => action.oh_toggle = true,
                '6' => action.perc_toggle = true,
                '7' => action.pattern = Some(SelPatt::Prev),
                '8' => action.pattern = Some(SelPatt::Next),
                '9' => action.scale = Some(SelScale::Next),
                '/' => {
                    if opt {
                        action.lead1 = Some(Lead1State::None);
                    } else {
                        action.lead0 = Some(Lead0State::None);
                    }
                }
                '*' => {
                    if opt {
                        action.lead1 = Some(Lead1State::Acid);
                    } else {
                        action.lead0 = Some(Lead0State::Arp);
                    }
                }
                '-' => {
                    if opt {
                        action.lead1 = Some(Lead1State::Psy);
                    } else {
                        action.lead0 = Some(Lead0State::Atm);
                    }
                }
                '+' => opt = true,
                '.' => action.arp_toggle = true,
                _ => {}
            }
        }
        action
    }
}
