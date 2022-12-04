fn get_total_calories(calorie_str: &str) -> i32 {
    calorie_str
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .sum()
}

fn problem_1(input: &str) {
    println!(
        "{}",
        input.split("\n\n").map(get_total_calories).max().unwrap()
    )
}

fn problem_2(input: &str) {
    let mut calories_per_elf: Vec<i32> = input.split("\n\n").map(get_total_calories).collect();
    let index_position = calories_per_elf.len() - 3;
    calories_per_elf
        .as_mut_slice()
        .select_nth_unstable(index_position);
    println!("{}", calories_per_elf.iter().rev().take(3).sum::<i32>());
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
            eprintln!("<PROBLEM_NUMBER> must be either \"1\" or \"2\"");
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

/*
fn problem_1(input: &str) {}
fn problem_2(input: &str) {}

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
        problem_1(&input);
    }
}



 */
