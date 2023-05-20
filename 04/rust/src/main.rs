mod a;
mod b;
mod range;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <a/b> <input>", args[0]);
        std::process::exit(1);
    }
    let part = args[1].as_str();
    let input_file = args[2].as_str();
    let result = match part {
        "a" => a::main(input_file),
        "b" => b::main(input_file),
        _ => panic!("Invalid part"),
    };

    match result {
        Ok(score) => println!("Your score is {}", score),
        Err(e) => println!("Error: {}", e),
    }

}
