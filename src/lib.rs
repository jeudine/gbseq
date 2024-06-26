use crate::perc::Perc;
use crate::state::LeadState;
use action::handle;
use clock::{clock_gen, compute_period_us};
use message::messages_gen;
use midir::{ConnectError, InitError, MidiOutput, MidiOutputConnection, MidiOutputPort};
pub use pattern::Note;
pub use pattern::Pattern;
use promptly::{prompt, prompt_default, ReadlineError};
pub use state::Stage;
use state::State;
pub use state::Transition;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::spawn;
use std::time::Instant;
use thiserror::Error;
pub mod acid;
mod action;
mod clock;
mod lead;
mod message;
pub mod pattern;
pub mod perc;
pub mod sequence;
mod state;
mod trig;

#[derive(Error, Debug)]
pub enum TSeqError {
    #[error("Failed to create a midi output [{}: {}]\n\t{0}", file!(), line!())]
    MidiInit(#[from] InitError),
    #[error("No output found [{}: {}]", file!(), line!())]
    NoOutput(),
    #[error("Read line [{}: {}]", file!(), line!())]
    ReadLine(#[from] ReadlineError),
    #[error("Invalid port number selected [{}: {}]", file!(), line!())]
    PortNumber(),
    #[error("Midi output issue [{}: {}]", file!(), line!())]
    MidiOutput(#[from] ConnectError<MidiOutput>),
}

struct Channel {
    conn: MidiOutputConnection,
    period_us: u64,
    timestamp: Instant,
    update_timestamp: bool,
    bpm_step: u32,
    step: u32,
}

pub struct Step {}

pub fn run(channel_id: u8, patterns: Vec<Pattern>, perc: Perc) -> Result<(), TSeqError> {
    let midi_out = MidiOutput::new("out")?;
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err(TSeqError::NoOutput()),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }

            let port_number: usize = prompt_default("Select output port", 0)?;
            match out_ports.get(port_number) {
                None => return Err(TSeqError::PortNumber()),
                Some(x) => x,
            }
        }
    };

    let conn = midi_out.connect(out_port, "output connection")?;

    let channel = Channel {
        conn,
        period_us: compute_period_us(patterns[0].bpm),
        timestamp: Instant::now(),
        update_timestamp: true,
        bpm_step: 0,
        step: 0,
    };
    let channel_arc = Arc::new((Mutex::new(channel), Condvar::new()));

    let mut infos = (patterns[0].root, patterns[0].bpm, LeadState::None);

    let state = State::new(patterns, perc);

    let state_arc = Arc::new(Mutex::new(state));

    // Clock
    let channel_arc_1 = channel_arc.clone();
    let _ = spawn(move || clock_gen(&channel_arc_1));

    // Messages
    let channel_arc_1 = channel_arc.clone();
    let state_arc_1 = state_arc.clone();
    let _ = spawn(move || messages_gen(&channel_arc_1, &state_arc_1, channel_id - 1));

    loop {
        let s = format!("[{} {} {}]", infos.0.get_str(), infos.1, infos.2);
        let s: String = prompt(s)?;
        if let Some(i) = handle(&s, &channel_arc, &state_arc) {
            infos = i;
        } else {
            break;
        }
    }
    Ok(())
}

pub fn log_send(conn: &mut MidiOutputConnection, message: &[u8]) {
    match conn.send(message) {
        Err(x) => eprintln!("[ERROR] {} (message: {:?})", x, message),
        _ => {}
    }
}
