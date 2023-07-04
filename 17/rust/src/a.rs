use crate::chamber;

pub fn main(contents: String) -> Result<String, String> {
    let mut chamber = chamber::Chamber::new();
    chamber.drop_rock();
    Ok(format!("{}", chamber))
}