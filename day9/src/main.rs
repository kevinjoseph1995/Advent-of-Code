use std::collections::HashSet;

type Position = (i32, i32);

struct SplitDeltaPosition {
    delta_position: Position,
    count: i32,
}

impl SplitDeltaPosition {
    fn new(base_position: Position) -> Self {
        let count;
        let mut delta_position = base_position;
        if base_position.0 != 0 {
            count = base_position.0.abs();
            delta_position.0 = delta_position.0 / count;
        } else {
            count = delta_position.1.abs();
            delta_position.1 = delta_position.1 / count;
        }
        SplitDeltaPosition {
            delta_position,
            count,
        }
    }
}

impl Iterator for SplitDeltaPosition {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            self.count -= 1;
            Some(self.delta_position)
        }
    }
}

fn problem_1(input: &str) {
    let mut head_position: Position = (0, 0);
    let mut tail_position: Position = (0, 0);

    let mut tail_positions = HashSet::new();
    tail_positions.insert((0, 0));

    for tail_pos in input
        .lines()
        .map(|line| -> Position {
            let mut split = line.split(" ");

            let direction = split.next().unwrap();
            let delta = split.next().unwrap().parse::<i32>().unwrap();
            match direction {
                "L" => (-(delta as i32), 0),
                "R" => ((delta as i32), 0),
                "U" => (0, (delta as i32)),
                "D" => (0, -(delta as i32)),
                _ => {
                    panic!("Unreachable")
                }
            }
        })
        .flat_map(|delta_position| SplitDeltaPosition::new(delta_position))
        .map(|delta_position| -> Position {
            head_position.0 += delta_position.0;
            head_position.1 += delta_position.1;
            head_position
        })
        .map(|head_position: Position| {
            let delta_x = head_position.0 - tail_position.0;
            let delta_y = head_position.1 - tail_position.1;
            if delta_x.abs() == 2 {
                if delta_y.abs() == 1 {
                    tail_position.1 += delta_y;
                }
                tail_position.0 += delta_x / delta_x.abs();
            }
            if delta_y.abs() == 2 {
                if delta_x.abs() == 1 {
                    tail_position.0 += delta_x / delta_x.abs();
                }
                tail_position.1 += delta_y / delta_y.abs();
            }
            tail_position
        })
    {
        tail_positions.insert(tail_pos);
    }

    println!("{}", tail_positions.len());
}

fn problem_2(input: &str) {
    let mut head_position: Position = (0, 0);
    let mut body = [(0, 0); 9];

    let mut tail_positions = HashSet::new();
    tail_positions.insert((0, 0));

    for tail_pos in input
        .lines()
        .map(|line| -> Position {
            let mut split = line.split(" ");

            let direction = split.next().unwrap();
            let delta = split.next().unwrap().parse::<i32>().unwrap();
            match direction {
                "L" => (-(delta as i32), 0),
                "R" => ((delta as i32), 0),
                "U" => (0, (delta as i32)),
                "D" => (0, -(delta as i32)),
                _ => {
                    panic!("Unreachable")
                }
            }
        })
        .flat_map(|delta_position| SplitDeltaPosition::new(delta_position))
        .map(|delta_position| -> Position {
            head_position.0 += delta_position.0;
            head_position.1 += delta_position.1;
            head_position
        })
        .map(|head_position: Position| -> Position {
            let compute_new_position = |old: Position, next: Position| -> Position {
                let mut new = old;
                let delta_x = next.0 - old.0;
                let delta_y = next.1 - old.1;
                if delta_x.abs() == 2 {
                    if delta_y.abs() == 1 {
                        new.1 += delta_y;
                    }
                    new.0 += delta_x / delta_x.abs();
                }
                if delta_y.abs() == 2 {
                    if delta_x.abs() == 1 {
                        new.0 += delta_x / delta_x.abs();
                    }
                    new.1 += delta_y / delta_y.abs();
                }
                new
            };

            let mut next = head_position;

            for i in 0..body.len() {
                body[i] = compute_new_position(body[i], next);
                next = body[i];
            }
            *body.last().unwrap()
        })
    {
        tail_positions.insert(tail_pos);
    }

    println!("{}", tail_positions.len());
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
