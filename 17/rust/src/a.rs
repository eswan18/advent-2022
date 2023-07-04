use crate::chamber;

pub fn main(contents: String) -> Result<String, String> {
    let mut chamber = chamber::Chamber::new(contents.trim());
    // Do this 2022 times.
    for i in 0..2022 {
        println!("Dropping rock {}", i);
        chamber.drop_rock();
    }
    Ok((chamber.highest_point() + 1).to_string())
}