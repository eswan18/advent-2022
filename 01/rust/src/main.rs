use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};


fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <a> <input_file>", args[0]);
        println!("Where <a> is either 'a' or 'b', and <input_file> is the input file");
        std::process::exit(1);
    }
    let part = &args[1];
    let filename = &args[2];
    println!("Reading from file: {}", filename);

    // Open the file
    let file = File::open(&Path::new(filename))?;

    // Create a vector to store the lines
    let mut lines: Vec<Vec<i32>> = Vec::new();
    lines.push(Vec::new());

    // Use a BufReader to read the file line by line
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line == "" {
            lines.push(Vec::new());
        } else {
            let number = line.parse::<i32>().unwrap();
            lines.last_mut().unwrap().push(number);

        }
    }

    // Sum up the vectors
    let mut sums: Vec<i32> = Vec::new();
    lines.iter().for_each(|line| {
        let mut sum = 0;
        line.iter().for_each(|number| {
            sum += number;
        });
        sums.push(sum);
    });

    // Get the maximum sum
    if part == "a" {
        let max_sum = sums.iter().max().unwrap();
        println!("The maximum sum is: {}", max_sum);
    } else {
        // Find the top 3 sums
        sums.sort();
        sums.reverse();
        let total: i32 = sums[0..3].iter().sum();
        println!("The total of the top 3 sums is: {}", total);
    }

    Ok(())
}
