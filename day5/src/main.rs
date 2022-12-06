fn compute_number_of_stacks(input: &str) -> usize {
    let line = input.lines().next().unwrap();
    (line.len() + 1) / 4
}

fn get_stack_base_line_index(input: &str) -> usize {
    input.lines().position(|x| x == "").unwrap() - 2
}

fn parse_diagram_line(line: &str) -> Vec<(usize, char)> {
    let mut results = Vec::<(usize, char)>::new();
    for (index, ch) in line.chars().enumerate() {
        if ch.is_alphabetic() {
            results.push(((index - 1) / 4, ch));
        }
    }
    results
}
#[derive(Debug)]
struct Command {
    num: usize,
    src: usize,
    dst: usize,
}

fn parse_command_line(line: &str) -> Command {
    let num;
    let src;
    let dst;
    let mut current_index = 0 as usize;
    {
        // Parse size
        current_index = current_index + 5;
        let start = current_index;
        for ch in line.chars().skip(start + 1) {
            current_index += 1;
            if ch == ' ' {
                break;
            }
        }
        let end = current_index;
        num = line[start..end].parse::<usize>().unwrap();
        current_index += 6;
    }
    {
        let start = current_index;
        for ch in line.chars().skip(start + 1) {
            current_index += 1;
            if ch == ' ' {
                break;
            }
        }
        let end = current_index;
        src = line[start..end].parse::<usize>().unwrap();
        current_index += 4;
    }
    {
        let start = current_index;
        dst = line[start..].parse::<usize>().unwrap();
    }
    Command { num, src, dst }
}

fn generate_stack_from_diagram(input: &str, base_index: usize) -> Vec<Vec<char>> {
    let number_of_stacks = compute_number_of_stacks(input);
    let mut stacks = vec![Vec::<char>::new(); number_of_stacks];
    let diagram_lines = input.lines().take(base_index + 1).collect::<Vec<&str>>();
    for line in diagram_lines.into_iter().rev() {
        for (stack_index, container) in parse_diagram_line(line) {
            stacks[stack_index].push(container);
        }
    }
    stacks
}

fn problem_1(input: &str) {
    let base_index = get_stack_base_line_index(input);
    let mut stacks = generate_stack_from_diagram(input, base_index);
    for line in input.lines().skip(base_index + 3) {
        let command = parse_command_line(line);
        for _ in 0..command.num {
            let popped;
            {
                popped = stacks[command.src - 1].pop().unwrap();
            }
            stacks[command.dst - 1].push(popped);
        }
    }
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!("")
}

fn problem_2(input: &str) {
    let base_index = get_stack_base_line_index(input);
    let mut stacks = generate_stack_from_diagram(input, base_index);
    for line in input.lines().skip(base_index + 3) {
        let command = parse_command_line(line);

        let mut temp: Vec<char> = Vec::new();
        for _ in 0..command.num {
            temp.push(stacks[command.src - 1].pop().unwrap());
        }
        for ch in temp.into_iter().rev() {
            stacks[command.dst - 1].push(ch);
        }
    }
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!("")
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
