fn get_range_from_range_str(range_str: &str) -> (usize, usize) {
    let mut split = range_str.split("-");
    let lower_str = split.next().unwrap();
    let upper_str = split.next().unwrap();
    (
        lower_str.parse::<usize>().unwrap(),
        upper_str.parse::<usize>().unwrap(),
    )
}
fn get_intersection_size(range1: (usize, usize), range2: (usize, usize)) -> i64 {
    let intersection_lower = std::cmp::max(range1.0, range2.0) as i64;
    let intersection_upper = std::cmp::min(range1.1, range2.1) as i64;
    intersection_upper - intersection_lower + 1 as i64
}
fn fully_contains(range1: (usize, usize), range2: (usize, usize)) -> bool {
    let intersection_size = get_intersection_size(range1, range2);
    if intersection_size > 0 {
        let range_1_size = (range1.1 - range1.0 + 1) as i64;
        let range_2_size = (range2.1 - range2.0 + 1) as i64;
        if intersection_size == range_1_size || intersection_size == range_2_size {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}
fn problem_1(input: &str) {
    println!(
        "{}",
        input
            .lines()
            .map(|line| {
                let mut split = line.split(",");
                let range1_str = split.next().unwrap();
                let range2_str = split.next().unwrap();
                (range1_str, range2_str)
            })
            .map(|(range1_str, range2_str)| {
                (
                    get_range_from_range_str(range1_str),
                    get_range_from_range_str(range2_str),
                )
            })
            .map(|(range1, range2)| fully_contains(range1, range2))
            .filter(|x| x.clone())
            .count()
    );
}

fn problem_2(input: &str) {
    println!(
        "{}",
        input
            .lines()
            .map(|line| {
                let mut split = line.split(",");
                let range1_str = split.next().unwrap();
                let range2_str = split.next().unwrap();
                (range1_str, range2_str)
            })
            .map(|(range1_str, range2_str)| {
                (
                    get_range_from_range_str(range1_str),
                    get_range_from_range_str(range2_str),
                )
            })
            .map(|(range1, range2)| get_intersection_size(range1, range2) > 0)
            .filter(|x| x.clone())
            .count()
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
