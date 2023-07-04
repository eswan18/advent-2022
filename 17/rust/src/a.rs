use crate::chamber;

pub fn main(contents: String) -> Result<String, String> {
    let mut chamber = chamber::Chamber::new();
    // Do this 2022 times.
    chamber.drop_rock();
    Ok(format!("{}", chamber.highest_point()))
}