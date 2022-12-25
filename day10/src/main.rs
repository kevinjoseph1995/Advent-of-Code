#[derive(Debug)]
enum OpCode {
    NoOp,
    AddX(i32),
}

struct CPUState<I>
where
    I: Iterator<Item = OpCode>,
{
    x: i32,
    cycle_count: i32,
    it: I,
    pending_delta: Option<i32>,
}

impl<I> Iterator for CPUState<I>
where
    I: Iterator<Item = OpCode>,
{
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pending_update) = self.pending_delta {
            self.cycle_count += 1;
            let x = self.x;
            self.x += pending_update;
            self.pending_delta = None;
            return Some((self.cycle_count, x));
        }
        if let Some(opcode) = self.it.next() {
            match opcode {
                OpCode::NoOp => {
                    self.cycle_count += 1;
                    Some((self.cycle_count, self.x))
                }
                OpCode::AddX(delta) => {
                    self.cycle_count += 1;
                    self.pending_delta = Some(delta);
                    Some((self.cycle_count, self.x))
                }
            }
        } else {
            None
        }
    }
}

trait CPUStateIterator: Iterator<Item = OpCode> + Sized {
    fn cpu_state(self) -> CPUState<Self> {
        CPUState {
            x: 1,
            cycle_count: 0,
            it: self,
            pending_delta: None,
        }
    }
}

impl<I: Iterator<Item = OpCode>> CPUStateIterator for I {}

fn problem_1(input: &str) {
    let mut sum = 0;
    for (cycle, x) in input
        .lines()
        .map(|line| {
            if line == "noop" {
                OpCode::NoOp
            } else {
                OpCode::AddX(line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap())
            }
        })
        .cpu_state()
    {
        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => sum += cycle * x,
            _ => {}
        }
    }
    println!("sum: {}", sum);
}

fn problem_2(input: &str) {
    for (cycle, x) in input
        .lines()
        .map(|line| {
            if line == "noop" {
                OpCode::NoOp
            } else {
                OpCode::AddX(line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap())
            }
        })
        .cpu_state()
    {
        let draw_index = (cycle - 1) % 40;

        let overlaps = || -> bool {
            for i in (x - 1)..=(x + 1) {
                if i <= 39 && draw_index == i {
                    return true;
                }
            }
            return false;
        };
        if overlaps() {
            print!("#")
        } else {
            print!(" ");
        }

        if cycle % 40 == 0 {
            println!("")
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
