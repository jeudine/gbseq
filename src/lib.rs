use midir::{ConnectError, InitError, MidiOutput, MidiOutputPort};
use promptly::{prompt_default, ReadlineError};
use std::thread::sleep;
use std::time::Duration;
use thiserror::Error;

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

pub struct Step {}

pub fn run() -> Result<(), TSeqError> {
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
	let mut conn_out = midi_out.connect(out_port, "output connection")?;
	{
		// Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
		let mut play_note = |note: u8, duration: u64| {
			const NOTE_ON_MSG: u8 = 0x90;
			const NOTE_OFF_MSG: u8 = 0x80;
			const VELOCITY: u8 = 0x64;
			// We're ignoring errors in here
			let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
			sleep(Duration::from_millis(duration * 150));
			let _ = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
			sleep(Duration::from_millis(duration * 150));
		};

		sleep(Duration::from_millis(4 * 150));

		play_note(66, 4);
		play_note(65, 3);
		play_note(63, 1);
		play_note(61, 6);
		play_note(59, 2);
		play_note(58, 4);
		play_note(56, 4);
		play_note(54, 4);
	}
	sleep(Duration::from_millis(150));
	println!("\nClosing connection");
	// This is optional, the connection would automatically be closed as soon as it goes out of scope
	conn_out.close();
	println!("Connection closed");
	Ok(())
}
