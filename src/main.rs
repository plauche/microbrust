mod utils;
mod microbrute;
mod state;
mod midi_interface;

use std::error::Error;
use state::State;
use midi_interface::MidiInterface;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description())
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut state = State::new();
    let mut midi_interface = MidiInterface::new();
    state = midi_interface.read_state(state).unwrap();

    microbrute::print_state(&state);
    state = midi_interface.set_state("note_priority", "last", state).unwrap();
    state = midi_interface.set_state("play", "note_on", state).unwrap();
    microbrute::print_state(&state);
    state = midi_interface.read_state(state).unwrap();
    microbrute::print_state(&state);
    Ok(())
}

