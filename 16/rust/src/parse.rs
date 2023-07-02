use std::collections::HashMap;

use crate::valve::Valve;
use regex::Regex;

pub fn parse(contents: String) -> Result<HashMap<String, Valve>, String> {
    let lines = contents.lines();
    let valves = lines.map(|line| parse_line(String::from(line))).collect::<Result<Vec<Valve>, String>>()?;
    let valves: HashMap<String, Valve> = valves
        .into_iter()
        .map(|v| (v.name.clone(), v))
        .collect::<HashMap<String, Valve>>();
    Ok(valves)
}

fn parse_line(line: String) -> Result<Valve, String> {
    let regex =
        match Regex::new(r"^Valve (.*) has flow rate=(.*); tunnels? leads? to valves? (.*)$") {
            Ok(r) => r,
            Err(e) => return Err(format!("Regex error: {}", e)),
        };
    let cap = match regex.captures(&line) {
        Some(c) => c,
        None => return Err(format!("Regex failed to match: {}", line)),
    };
    let name = cap
        .get(1)
        .ok_or_else(|| String::from("No pattern"))?
        .as_str()
        .to_string();
    let rate = cap
        .get(2)
        .ok_or_else(|| String::from("No pattern"))?
        .as_str()
        .parse::<usize>()
        .map_err(|e| format!("Failed to parse rate: {}", e))?;
    let linked_valves: Vec<&str> = cap
        .get(3)
        .ok_or_else(|| String::from("No pattern"))?
        .as_str()
        .split(",")
        .map(|s| s.trim())
        .collect();
    Ok(Valve::new(name, rate, linked_valves))
}
