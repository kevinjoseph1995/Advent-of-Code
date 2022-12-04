#[derive(PartialEq, Clone)]
enum Play {
    Rock,
    Paper,
    Scissor,
}

#[derive(PartialEq, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

fn map_action(action_str: &str) -> Play {
    match action_str {
        "A" | "X" => Play::Rock,
        "B" | "Y" => Play::Paper,
        "C" | "Z" => Play::Scissor,
        _ => {
            panic!();
        }
    }
}

fn map_outcome(action_str: &str) -> Outcome {
    match action_str {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => {
            panic!();
        }
    }
}

fn score(opponent_play: Play, own_play: Play) -> u32 {
    let mut score = match &own_play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissor => 3,
    };
    match own_play {
        Play::Rock => {
            if opponent_play == Play::Scissor {
                score += 6;
            } else if opponent_play == Play::Rock {
                score += 3;
            }
        }
        Play::Paper => {
            if opponent_play == Play::Rock {
                score += 6;
            } else if opponent_play == Play::Paper {
                score += 3;
            }
        }
        Play::Scissor => {
            if opponent_play == Play::Paper {
                score += 6;
            } else if opponent_play == Play::Scissor {
                score += 3;
            }
        }
    }
    return score;
}

fn score_from_outcome(opponent_play: Play, expected: Outcome) -> u32 {
    match opponent_play {
        Play::Rock => {
            if expected == Outcome::Win {
                return score(opponent_play, Play::Paper);
            } else if expected == Outcome::Lose {
                return score(opponent_play, Play::Scissor);
            } else {
                return score(opponent_play, Play::Rock);
            }
        }
        Play::Paper => {
            if expected == Outcome::Win {
                return score(opponent_play, Play::Scissor);
            } else if expected == Outcome::Lose {
                return score(opponent_play, Play::Rock);
            } else {
                return score(opponent_play, Play::Paper);
            }
        }
        Play::Scissor => {
            if expected == Outcome::Win {
                return score(opponent_play, Play::Rock);
            } else if expected == Outcome::Lose {
                return score(opponent_play, Play::Paper);
            } else {
                return score(opponent_play, Play::Scissor);
            }
        }
    }
}

fn problem_1(input: &str) {
    println!(
        "{}",
        input
            .split("\n")
            .map(|x| x.split(" ").collect::<Vec<&str>>())
            .map(|x| (map_action(x[0]), map_action(x[1])))
            .map(|x| score(x.0, x.1))
            .sum::<u32>()
    );
}
fn problem_2(input: &str) {
    println!(
        "{}",
        input
            .split("\n")
            .map(|x| x.split(" ").collect::<Vec<&str>>())
            .map(|x| (map_action(x[0]), map_outcome(x[1])))
            .map(|x| score_from_outcome(x.0, x.1))
            .sum::<u32>()
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
