use std::fmt::format;

use crate::packet::Packet;

pub fn main(contents: String) -> Result<String, String> {
    let pairs: Vec<(Packet, Packet)> = contents
        .trim()
        .split("\n\n")
        .map(|lines| {
            let lines: Vec<_> = lines.split("\n").collect();
            if lines.len() != 2 {
                return Err(format!("Invalid input: {}", lines.join("\n")));
            }
            let left_packet = Packet::build_from_text(lines[0])?;
            let right_packet = Packet::build_from_text(lines[1])?;
            Ok((left_packet, right_packet))
        })
        .collect::<Result<Vec<(Packet, Packet)>, String>>()?;

    let mut sum = 0;
    for (index, (p1, p2)) in pairs.iter().enumerate() {
        if p1 <= p2 {
            // Adjust for one-based indexing.
            sum += index + 1;
        }
    }
    Ok(sum.to_string())
}