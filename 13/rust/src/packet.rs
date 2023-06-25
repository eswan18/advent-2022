use crate::token::Token;
use std::fmt::Display;


#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    items: Vec<PacketItem>,
}

impl Packet {
    pub fn build_from_items(items: Vec<PacketItem>) -> Packet {
        Packet { items }
    }

    pub fn build_from_tokens(tokens: &mut Vec<Token>) -> Result<Self, String> {
        let mut items = Vec::new();
        // Opening bracket.
        match tokens[0] {
            Token::LB => tokens.remove(0),
            _ => return Err("Expected LB".to_string()),
        };
        loop {
            match tokens.remove(0) {
                Token::Comma => continue,
                Token::Number(n) => {
                    items.push(PacketItem::Number(n));
                }
                Token::LB => {
                    // This means we're starting a nested packet. We need to add the LB back so that the recursive parser can see it.
                    tokens.insert(0, Token::LB);
                    let inner_packet = Self::build_from_tokens(tokens)?;
                    items.push(PacketItem::Packet(inner_packet));
                }
                Token::RB => break,
            }
        }
        Ok(Packet { items })
    }

    pub fn build_from_text(text: &str) -> Result<Packet, String> {
        let mut tokens = Token::tokenize(text)?;
        Self::build_from_tokens(&mut tokens)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for i in 0..(self.items.len().max(other.items.len())) {
            let self_item = self.items.get(i);
            let other_item = other.items.get(i);
            match (self_item, other_item) {
                (Some(self_item), Some(other_item)) => {
                    let cmp = self_item.cmp(other_item);
                    if cmp != std::cmp::Ordering::Equal {
                        return cmp;
                    }
                }
                (Some(_), None) => return std::cmp::Ordering::Greater,
                (None, Some(_)) => return std::cmp::Ordering::Less,
                (None, None) => return std::cmp::Ordering::Equal,
            }
            
        }
        std::cmp::Ordering::Equal
    } 
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other)) 
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PacketItem {
    Number(i32),
    Packet(Packet),
}

impl Ord for PacketItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PacketItem::Number(n1), PacketItem::Number(n2)) => n1.cmp(n2),
            (PacketItem::Packet(p1), PacketItem::Packet(p2)) => p1.cmp(p2),
            (PacketItem::Number(n), PacketItem::Packet(p)) => {
                let n_as_packet = Packet::build_from_items(vec![PacketItem::Number(*n)]);
                n_as_packet.cmp(p)
            }
            (PacketItem::Packet(p), PacketItem::Number(n)) => {
                let n_as_packet = Packet::build_from_items(vec![PacketItem::Number(*n)]);
                p.cmp(&n_as_packet)
            }
        }
    }
}

impl PartialOrd for PacketItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for PacketItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketItem::Number(n) => write!(f, "{}", n),
            PacketItem::Packet(p) => write!(f, "{}", p),
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for item in &self.items {
            if !first {
                write!(f, ",")?;
            } else {
                first = false
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let p = Packet::build_from_items(vec![PacketItem::Number(1)]);
        assert_eq!(format!("{}", p), "[1]");

        let p = Packet::build_from_items(vec![PacketItem::Packet(Packet::build_from_items(vec![PacketItem::Number(1)]))]);
        assert_eq!(format!("{}", p), "[[1]]");
    }

    
    #[test]
    fn test_build_from_tokens_simple() {
        let text = "[1,2,3]";
        let packet = Packet::build_from_text(text).unwrap();
        let as_string = format!("{}", packet);
        assert_eq!(as_string, text);
    }

    #[test]
    fn test_build_from_tokens_nested() {
        let text = "[1,[1,2,3],3]";
        let packet = Packet::build_from_text(text).unwrap();
        let as_string = format!("{}", packet);
        assert_eq!(as_string, text);

        let text = "[[],1,[1,2],3]";
        let packet = Packet::build_from_text(text).unwrap();
        let as_string = format!("{}", packet);
        assert_eq!(as_string, text);
    }

    #[test]
    fn test_packet_order_simple() {
        let p1 = Packet::build_from_text("[1,2,3]").unwrap();
        let p2 = Packet::build_from_text("[1,2,4]").unwrap();
        assert!(p1 < p2);
        assert!(p2 >= p1);
        assert!(p2 >= p2);
        assert!(p1 == p1);

        let p1 = Packet::build_from_text("[1]").unwrap();
        let p2 = Packet::build_from_text("[0,2,4]").unwrap();
        assert!(p1 > p2);

        let p1 = Packet::build_from_text("[0]").unwrap();
        let p2 = Packet::build_from_text("[0,2,4]").unwrap();
        assert!(p1 < p2);

        let p1 = Packet::build_from_text("[7,7,7,7]").unwrap();
        let p2 = Packet::build_from_text("[7,7,7]").unwrap();
        assert!(p1 > p2);
    }

    #[test]
    fn test_packet_order_nested() {
        let p1 = Packet::build_from_text("[[4,4],4,4]").unwrap();
        let p2 = Packet::build_from_text("[[4,4],4,4,4]").unwrap();
        assert!(p1 < p2);

        let p1 = Packet::build_from_text("[9]").unwrap();
        let p2 = Packet::build_from_text("[[8,7,6]]").unwrap();
        assert!(p1 > p2);

        let p1 = Packet::build_from_text("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();
        let p2 = Packet::build_from_text("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap();
        assert!(p1 > p2);

        let p1 = Packet::build_from_text("[[[]]]").unwrap();
        let p2 = Packet::build_from_text("[[]]").unwrap();
        assert!(p1 > p2);
    }
}
