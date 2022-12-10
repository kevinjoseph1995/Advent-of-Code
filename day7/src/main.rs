#[derive(Debug)]
enum FileSystemCommand<'a> {
    ChangeDir(&'a str),
    ListDir(Vec<&'a str>),
}

#[derive(Debug)]
struct Node {
    name: String,
    parent_index: Option<usize>,
    discovered_size: usize,
    fully_discovered: bool,
    explored: bool,
    children: Vec<usize>,
}

#[derive(Debug)]
struct FilesystemTree {
    nodes: Vec<Node>,
    current_index: usize,
}

impl FilesystemTree {
    fn new() -> Self {
        FilesystemTree {
            current_index: 0,
            nodes: vec![Node {
                name: String::from("/"),
                parent_index: None,
                discovered_size: 0,
                fully_discovered: false,
                explored: false,
                children: vec![],
            }],
        }
    }
    fn change_current(&mut self, dst: &str) {
        if dst == "/" {
            self.current_index = 0;
        } else if dst == ".." {
            let current_node = &self.nodes[self.current_index];
            assert!(
                current_node.parent_index.is_some(),
                "Already at root, parent does not exist"
            );
            self.current_index = current_node.parent_index.unwrap();
        } else {
            let current_node = &self.nodes[self.current_index];
            for child_index in &current_node.children {
                let child_node = &self.nodes[*child_index];
                if child_node.name == dst {
                    self.current_index = *child_index;
                    return;
                }
            }
            panic!("Destination node:{} not in children", dst);
        }
    }
    fn consume_directory_contents(&mut self, directory_contents: Vec<&str>) {
        if self.nodes[self.current_index].explored {
            return;
        }
        self.nodes[self.current_index].explored = true;
        for directory_entry in directory_contents {
            if let Some(suffix) = directory_entry.strip_prefix("dir ") {
                // Add child to node pool
                self.nodes.push(Node {
                    name: String::from(suffix),
                    parent_index: Some(self.current_index),
                    discovered_size: 0,
                    fully_discovered: false,
                    explored: false,
                    children: vec![],
                });
                // Get the index
                let child_index = self.nodes.len() - 1;
                self.nodes[self.current_index].children.push(child_index); // Add as child of current node
            } else {
                let mut split = directory_entry.split(" ");
                let size_str = split.next().unwrap();
                let file_size = size_str.parse::<usize>().unwrap();
                self.nodes[self.current_index].discovered_size += file_size;
            }
        }
        // Fixup the sizes of all the ancestors if the node we just explored has it's size completely determined
        if self.nodes[self.current_index].children.is_empty() {
            let mut current_index = self.current_index;
            self.nodes[current_index].fully_discovered = true;
            while let Some(parent_index) = self.nodes[current_index].parent_index {
                self.nodes[parent_index].discovered_size +=
                    self.nodes[current_index].discovered_size;
                // Iterate through other siblings
                let mut parent_fully_discovered = true;
                for child_index in &self.nodes[parent_index].children {
                    if !self.nodes[*child_index].fully_discovered {
                        parent_fully_discovered = false;
                        break;
                    }
                }
                if parent_fully_discovered {
                    self.nodes[parent_index].fully_discovered = true;
                    current_index = parent_index;
                } else {
                    break;
                }
            }
        }
    }
}

struct FileSystemCommandExtractor<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    iter: std::iter::Peekable<I>,
}

impl<'a, I> FileSystemCommandExtractor<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    fn construct_filesystem_tree(&mut self) -> FilesystemTree {
        let mut tree = FilesystemTree::new();
        while let Some(command) = self.next() {
            match command {
                FileSystemCommand::ChangeDir(dst) => {
                    tree.change_current(dst);
                }
                FileSystemCommand::ListDir(children) => {
                    tree.consume_directory_contents(children);
                }
            }
        }
        return tree;
    }
}

impl<'a, I> Iterator for FileSystemCommandExtractor<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    type Item = FileSystemCommand<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.iter.next();
        if let None = line {
            return None;
        }
        let line = line.unwrap();
        if let Some(suffix) = line.strip_prefix("$ cd ") {
            return Some(FileSystemCommand::ChangeDir(suffix));
        }

        if line == "$ ls" {
            let mut ls_results: Vec<&str> = Vec::new();
            loop {
                let next_line = self.iter.peek();
                match next_line {
                    Some(next_line) => {
                        if (*next_line).starts_with("$") {
                            break;
                        }
                        let line = self.iter.next().unwrap();
                        ls_results.push(line);
                    }
                    None => break,
                }
            }
            return Some(FileSystemCommand::ListDir(ls_results));
        }
        panic!("Unreachable, iterator over invalid malformed lines");
    }
}

trait CommandIterator<'a>: Iterator<Item = &'a str> + Sized {
    fn commands(self) -> FileSystemCommandExtractor<'a, Self> {
        FileSystemCommandExtractor {
            iter: self.peekable(),
        }
    }
}

impl<'a, I: Iterator<Item = &'a str>> CommandIterator<'a> for I {}

fn problem_1(input: &str) {
    println!(
        "{}",
        input
            .lines()
            .into_iter()
            .commands()
            .construct_filesystem_tree()
            .nodes
            .iter()
            .map(|node| node.discovered_size)
            .filter(|size| *size <= 100000)
            .sum::<usize>()
    );
}

fn problem_2(input: &str) {
    let mut nodes = input
        .lines()
        .into_iter()
        .commands()
        .construct_filesystem_tree()
        .nodes;
    nodes.sort_by(|a, b| a.discovered_size.cmp(&b.discovered_size));
    let total_free_space = 70000000 - nodes.last().unwrap().discovered_size;
    let required_to_delete = 30000000 - total_free_space;
    for node in nodes {
        if node.discovered_size >= required_to_delete {
            println!(
                "Node to delete:{}, has size {}",
                node.name, node.discovered_size
            );
            return;
        }
    }
    panic!("Did not find node to delete");
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
