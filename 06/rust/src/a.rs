const WINDOW_LENGTH: usize = 4;

pub fn main(contents: String) -> Result<String, String> {
    for window_end in WINDOW_LENGTH..contents.len() {
        let window_start = window_end - WINDOW_LENGTH;
        let window = &contents[window_start..window_end];
        if all_chars_unique(window) {
            // We use one-based indexing in this madhouse, but ranges like [0, 4] are
            // exclusive on the right, which cancels out the need to add one.
            return Ok(window_end.to_string());
        }
    }
    

    Err(String::from("No answer found"))
}

fn all_chars_unique(s: &str) -> bool {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.dedup();
    chars.len() == s.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_chars_unique() {
        assert!(all_chars_unique("abc"));
        assert!(!all_chars_unique("abca"));
        assert!(!all_chars_unique("abbd"));
        assert!(all_chars_unique("a"));
        assert!(all_chars_unique(""));
    }

    #[test]
    fn test_main() {
        let contents_and_answers = vec![
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", "5"),
            ("nppdvjthqldpwncqszvftbrmjlhg", "6"),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10"),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11"),
        ];
        for (content, answer) in contents_and_answers {
            let actual_answer = main(content.to_string()).unwrap();
            assert_eq!(actual_answer, answer, "Failed on {}", content);
        }
    }
}