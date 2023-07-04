use crate::chamber;

pub fn main(contents: String) -> Result<String, String> {
    let mut chamber = chamber::Chamber::new(contents.trim());
    // Do this 2022 times.
    chamber.drop_rock();
    chamber.drop_rock();
    Ok("".to_string())
}