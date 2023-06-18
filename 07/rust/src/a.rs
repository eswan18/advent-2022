use crate::parse;

pub fn main(contents: String) -> Result<String, String> {
    parse::parse(contents)?;

    Ok(String::from("Hello"))
}

