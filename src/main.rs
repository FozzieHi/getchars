use rpassword::prompt_password;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: getchars <chars>");
        eprintln!("Example: getchars 1,2,4");
        std::process::exit(1);
    }

    let input_string = prompt_password("String: ").expect("Failed to read input.");

    let chars: Vec<char> = input_string.chars().collect();

    let positions = args[1]
        .split(',')
        .filter_map(|p| p.parse::<usize>().ok())
        .collect::<Vec<_>>();

    for &pos in &positions {
        if pos > 0 && pos <= chars.len() {
            print!("{}", chars[pos - 1]);
        } else {
            eprintln!("Position {} is out of bounds.", pos);
        }
    }

    println!();
}
