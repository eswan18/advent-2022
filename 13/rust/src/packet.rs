use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    LB, // [
    RB, // ]
    Number(i32),
    Comma, // ,
}

impl Token {
    pub fn tokenize(text: &str) -> Result<Vec<Self>, String> {
        let chars: Vec<char> = text.chars().collect();
        let mut tokens = Vec::new();
        let mut pos = 0;
        while pos < chars.len() {
            match chars[pos] {
                '[' => {
                    tokens.push(Token::LB);
                    pos += 1;
                }
                ']' => {
                    tokens.push(Token::RB);
                    pos += 1;
                }
                ',' => {
                    tokens.push(Token::Comma);
                    pos += 1;
                }
                c if c.is_digit(10) => {
                    let mut pos_range = (pos, pos + 1);
                    // There could be more than just one digit; consume all of them.
                    if pos_range.1 < chars.len() && chars[pos_range.1].is_digit(10) {
                        pos_range.1 += 1;
                    }
                    let number_str: String = chars[pos_range.0..pos_range.1].iter().collect();
                    let number: i32 = number_str.parse().map_err(|_| "Unable to parse number")?;
                    tokens.push(Self::Number(number));
                    pos = pos_range.1;
                }
                _ => return Err("invalid token".to_string()),
            }
        };
        Ok(tokens)
    }
}

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

#[derive(Debug, PartialEq, Eq)]
pub enum PacketItem {
    Number(i32),
    Packet(Packet),
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
    fn test_tokenize() {
        let tokens = Token::tokenize("[1,2,3]").unwrap();
        assert_eq!(tokens, vec![Token::LB, Token::Number(1), Token::Comma, Token::Number(2), Token::Comma, Token::Number(3), Token::RB]);
        let tokens = Token::tokenize("2,12][").unwrap();
        assert_eq!(tokens, vec![Token::Number(2), Token::Comma, Token::Number(12), Token::RB, Token::LB]);
    }
    
    #[test]
    fn test_build_from_tokens() {
        let text = "[1,2,3]";
        let mut tokens = Token::tokenize(text).unwrap();
        let packet = Packet::build_from_tokens(&mut tokens).unwrap();
        let as_string = format!("{}", packet);
        assert_eq!(as_string, text);

        let text = "[1,[1],3]";
        let mut tokens = Token::tokenize(text).unwrap();
        let packet = Packet::build_from_tokens(&mut tokens).unwrap();
        let as_string = format!("{}", packet);
        assert_eq!(as_string, text);
    }
}
