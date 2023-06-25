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
        let mut chars: Vec<char> = text.chars().collect();
        let mut tokens = Vec::new();
        let pos = 0;
        while pos < chars.len() {
            match chars[pos] {
                '[' => tokens.push(Token::LB),
                ']' => tokens.push(Token::RB),
                ',' => tokens.push(Token::RB),
                c if c.is_digit(10) => {
                    let mut pos_range = (pos, pos + 1);
                    // There could be more than just one digit; consume all of them.
                    if pos_range.1 < chars.len() && chars[pos_range.1].is_digit(10) {
                        pos_range.1 += 1;
                    }
                    let number_str: String = chars[pos_range.0..pos_range.1].iter().collect();
                    let number: i32 = number_str.parse().map_err(|_| "Unable to parse number")?;
                    tokens.push(Self::Number(number))
                }
                _ => return Err("invalid token".to_string()),
            }
        };
        Ok(tokens)
    }
}

pub struct Packet {
    items: Vec<PacketItem>,
}

impl Packet {
    pub fn build_from_items(items: Vec<PacketItem>) -> Packet {
        Packet { items }
    }

    pub fn build_from_tokens(tokens: &mut Vec<Token>) -> Result<Packet, String> {
        Ok(Packet { items: vec![] })
    }

    pub fn build_from_text(text: &str) -> Result<Packet, String> {
        let mut tokens = Token::tokenize(text)?;
        Self::build_from_tokens(&mut tokens)
    }
}

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
        for item in &self.items {
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
}
