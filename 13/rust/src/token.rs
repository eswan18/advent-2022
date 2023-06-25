
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = Token::tokenize("[1,2,3]").unwrap();
        assert_eq!(tokens, vec![Token::LB, Token::Number(1), Token::Comma, Token::Number(2), Token::Comma, Token::Number(3), Token::RB]);
        let tokens = Token::tokenize("2,12][").unwrap();
        assert_eq!(tokens, vec![Token::Number(2), Token::Comma, Token::Number(12), Token::RB, Token::LB]);
    }
}