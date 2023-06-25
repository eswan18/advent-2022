use crate::packet::Packet;

pub fn main(contents: String) -> Result<String, String> {
    let mut packets = contents
        .trim()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| Packet::build_from_text(line))
        .collect::<Result<Vec<Packet>, String>>()?;
    let mut divider_packets = vec![
        Packet::build_from_text("[[2]]")?,
        Packet::build_from_text("[[6]]")?,
    ];
    packets.append(&mut divider_packets);
    packets.sort();

    // Figure out where the divider packets wound up.
    let divider_one_index = packets
        .iter()
        .position(|p| p == &Packet::build_from_text("[[2]]").unwrap())
        .ok_or(String::from("divider packet not found"))?;
    let divider_two_index = packets
        .iter()
        .position(|p| p == &Packet::build_from_text("[[6]]").unwrap())
        .ok_or(String::from("divider packet not found"))?;
    // Adjust for 1-based indexing.
    let signal = (divider_one_index + 1) * (divider_two_index + 1);
    Ok(signal.to_string())
}