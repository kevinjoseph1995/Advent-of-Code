#![feature(iter_array_chunks)]

fn get_index(ch: char) -> usize {
    let ch_int = ch as i64;
    let index = ch_int - ('a' as i64);
    if index >= 0 {
        return index as usize;
    } else {
        return (ch_int as usize) - ('A' as usize) + 26;
    }
}

fn get_priority(ch: char) -> usize {
    let index = get_index(ch);
    index + 1
}

fn get_char(index: usize) -> char {
    if index >= 26 {
        return char::from_u32(((index - 26) + 'A' as usize) as u32).unwrap();
    } else {
        return char::from_u32((index + 'a' as usize) as u32).unwrap();
    }
}

fn problem_1(input: &str) {
    fn common_char(first: &str, second: &str) -> char {
        let mut lookup_table = [0; 52];
        for ch in first.chars() {
            lookup_table[get_index(ch)] = 1;
        }
        for ch in second.chars() {
            if lookup_table[get_index(ch)] == 1 {
                return ch;
            }
        }
        return '\0';
    }
    println!(
        "{:?}",
        input
            .lines()
            .map(|line| {
                let mid = line.len() / 2;
                (&line[0..mid], &line[mid..])
            })
            .map(|pair| common_char(pair.0, pair.1))
            .map(|ch| get_priority(ch))
            .sum::<usize>()
    )
}

fn find_common<const N: usize>(group: &[&str; N]) -> char {
    let mut lookup_table = [[false; 52]; 3];
    for i in 0..N {
        let part = group[i];
        for ch in part.chars() {
            lookup_table[i][get_index(ch)] = true;
        }
    }
    for i in 0..52 {
        lookup_table[0][i] = lookup_table[0][i] && lookup_table[1][i] && lookup_table[2][i];
        if lookup_table[0][i] {
            return get_char(i);
        }
    }
    return '\0';
}

fn problem_2(input: &str) {
    println!(
        "{}",
        input
            .lines()
            .array_chunks::<3>()
            .map(|chunk| find_common(&chunk))
            .map(|common| get_priority(common))
            .sum::<usize>(),
    );
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
