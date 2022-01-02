use std::collections::HashMap;
use std::io::Result;

use intcode;
use read_input::read_text;

fn send_packet_to_nic(
    nics: &mut Vec<intcode::ProgramState>,
    receiver_addr: usize,
    outputs: &[i64; 3],
) {
    let receiver = nics.get_mut(receiver_addr).unwrap();
    receiver.inputs.push(outputs[1]);
    receiver.inputs.push(outputs[2]);
    receiver.inputs.push(-1);
}

fn main() -> Result<()> {
    let text = read_text("23/input.txt")?;

    let base_program = intcode::get_base_program(&text);
    let mut nics: Vec<intcode::ProgramState> = (0..50)
        .map(|n| intcode::ProgramState::new(&base_program, vec![n, -1]))
        .collect();

    let mut packets: HashMap<i64, (usize, [i64; 3])> = HashMap::new();

    let mut logged_first_nat_packet = false;
    let mut nat_packet = [0i64; 3];
    let mut sent_to_addr_zero = 0;

    'main: loop {
        let mut sent_packets_this_step = false;
        for address in 0..nics.len() {
            let ai64 = address as i64;
            // put an empty packet in for this address
            if !packets.contains_key(&ai64) {
                packets.insert(ai64, (0, [0, 0, 0]));
            }

            // get the current index to set output from incode, as well as the array
            let (output_idx, outputs) = packets.get_mut(&ai64).unwrap();
            let nic = nics.get_mut(address).unwrap();
            let result = intcode::run_step(nic, false);
            if let Some(value) = result.0 {
                outputs[*output_idx] = value;

                // on last output
                if *output_idx == 2 {
                    let receiver_addr = outputs[0] as usize;
                    // send the packet data
                    if receiver_addr < 50 {
                        sent_packets_this_step = true;
                        send_packet_to_nic(&mut nics, receiver_addr, outputs);
                    } else if receiver_addr == 255 {
                        if !logged_first_nat_packet {
                            logged_first_nat_packet = true;
                            println!("p1 {:?}", outputs);
                        }
                        nat_packet[0] = outputs[0];
                        nat_packet[1] = outputs[1];
                        nat_packet[2] = outputs[2];
                    }
                    *output_idx = 0;
                } else {
                    *output_idx += 1;
                }
            }
        }

        let idle_nics = nics
            .iter()
            .filter(|nic| {
                nic.inputs_index >= nic.inputs.len() - 1 && *nic.inputs.last().unwrap() == -1
            })
            .count();

        if !sent_packets_this_step && logged_first_nat_packet && idle_nics == nics.len() {
            println!("sending packet to 0 {:?}", nat_packet);
            send_packet_to_nic(&mut nics, 0, &nat_packet);
            // if nat_packet[2] == sent_to_addr_zero {
            //     println!("p2 {}", sent_to_addr_zero);
            //     break 'main;
            // } else {
            //     sent_to_addr_zero = nat_packet[2];
            // }
        }
    }

    Ok(())
}
