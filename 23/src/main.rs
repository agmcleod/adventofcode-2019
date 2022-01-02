use std::collections::HashMap;
use std::io::Result;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

use intcode;
use read_input::read_text;

type Packet = (i64, i64, i64);

fn main() -> Result<()> {
    let text = read_text("23/input.txt")?;

    let base_program = intcode::get_base_program(&text);
    let nics: Vec<intcode::ProgramState> = (0..50)
        .map(|n| intcode::ProgramState::new(&base_program, vec![n, -1]))
        .collect();

    // let barrier = Arc::new(Barrier::new(nics.len()));

    let mut handlers = Vec::new();
    let mut senders = Vec::new();
    let mut receivers = HashMap::new();
    for address in 0..50 {
        let (sender, receiver) = channel::<Packet>();
        senders.push(sender);
        receivers.insert(address as i64, receiver);
    }

    for mut nic in nics {
        let address = nic.inputs[0];
        // let barrier = barrier.clone();
        let receiver = receivers.remove(&address).unwrap();
        let senders = senders.clone();
        let handler = thread::spawn(move || {
            let mut output_count = 0;
            let mut outputs = [0, 0, 0];

            intcode::run_program(&mut nic, false, |state, value| {
                outputs[output_count] = value;

                if output_count == 2 {
                    let address = outputs[0];
                    if address < 50 {
                        if address < 0 {
                            panic!("Address was less than zero {}", address);
                        }
                        let sender = senders.get(address as usize).unwrap();
                        sender.send((outputs[0], outputs[1], outputs[2])).unwrap();
                    }
                    output_count = 0;
                } else {
                    output_count += 1;
                }

                // barrier.wait();
                false
            });
        });

        handlers.push(handler);
    }

    for h in handlers {
        h.join().unwrap();
    }

    Ok(())
}
