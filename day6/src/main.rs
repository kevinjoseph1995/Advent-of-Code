fn is_header(window: &[u8]) -> bool {
    let mut lookup_table = [false; 26];
    for ch in window {
        let index = (ch - ('a' as u8)) as usize;
        if lookup_table[index] {
            return false;
        }
        lookup_table[index] = true;
    }
    return true;
}

fn problem_1(input: &str) {
    for (window_index, window) in input.as_bytes().windows(4).enumerate() {
        if is_header(window) {
            println!("{}", window_index + 4);
            break;
        }
    }
}

fn problem_2(input: &str) {
    for (window_index, window) in input.as_bytes().windows(14).enumerate() {
        if is_header(window) {
            println!("{}", window_index + 14);
            break;
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("day1 <PROBLEM_NUMBER> <INPUT_FILE>");
        return;
    }
    let problem;
    match args[1].as_str() {
        "1" | "2" => problem = String::from(&args[1]),
        _ => {
            return;
        }
    }

    let input_filename = &args[2];
    let input = std::fs::read_to_string(input_filename);
    if let Err(error) = input {
        eprintln!("{}", error);
        return;
    }
    let input = input.unwrap();
    if problem == "1" {
        problem_1(&input);
    } else {
        problem_2(&input);
    }
}
