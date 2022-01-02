use std::collections::HashMap;
use std::io::Result;

use intcode;
use read_input::read_text;

type Packet = (i64, i64, i64);

fn main() -> Result<()> {
    let text = read_text("23/input.txt")?;

    let base_program = intcode::get_base_program(&text);
    let mut nics: Vec<intcode::ProgramState> = (0..50)
        .map(|n| intcode::ProgramState::new(&base_program, vec![n, -1]))
        .collect();

    let mut packets: HashMap<i64, (usize, [i64; 3])> = HashMap::new();

    'main: loop {
        for address in 0..nics.len() {
            let ai64 = address as i64;
            if !packets.contains_key(&ai64) {
                packets.insert(ai64, (0, [0, 0, 0]));
            }

            let (output_idx, outputs) = packets.get_mut(&ai64).unwrap();
            let nic = nics.get_mut(address).unwrap();
            let result = intcode::run_step(nic, false);
            if let Some(value) = result.0 {
                outputs[*output_idx] = value;

                if *output_idx == 2 {
                    let receiver_addr = outputs[0] as usize;
                    if receiver_addr < 50 {
                        let receiver = nics.get_mut(receiver_addr).unwrap();
                        receiver.inputs.push(outputs[1]);
                        receiver.inputs.push(outputs[2]);
                        receiver.inputs.push(-1);
                    } else if receiver_addr == 255 {
                        println!("{:?}", outputs);
                        break 'main;
                    }
                    *output_idx = 0;
                } else {
                    *output_idx += 1;
                }
            }
        }
    }

    Ok(())
}
