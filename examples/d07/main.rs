fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    let mut current_path = vec![];
    let mut filesystem = Node::Folder {
        name: "/".to_string(),
        children: vec![],
    };

    for line in std::fs::read_to_string(puzzle_file).unwrap().lines() {
        process_line(line, &mut current_path, &mut filesystem);
    }
    println!("Part 1: {}", filesystem.filtered_size());
    let space_to_free = 30000000 - (70000000 - filesystem.size());
    let mut all_folders: Vec<&Node> = filesystem.find_folders();
    all_folders.sort_by_key(|n| n.size());
    println!(
        "Part 2: {}",
        all_folders
            .iter()
            .find(|n| n.size() >= space_to_free)
            .unwrap()
            .size()
    )
}
fn process_line<'a>(line: &'a str, current_path: &mut Vec<&'a str>, node: &mut Node) {
    let words: Vec<&str> = line.split_whitespace().collect();
    if words[0] == "$" {
        match words[1] {
            "ls" => (),
            "cd" => {
                if words[2] == ".." {
                    current_path.pop();
                } else if words[2] == "/" {
                    current_path.clear()
                } else {
                    current_path.push(words[2])
                }
            }
            _ => panic!("Invalid command in line [{}]", line),
        }
    } else {
        node.add_child(
            &current_path,
            if words[0] == "dir" {
                Node::Folder {
                    name: words[1].to_string(),
                    children: vec![],
                }
            } else {
                Node::File {
                    name: words[1].to_string(),
                    size: words[0].parse().unwrap(),
                }
            },
        )
    }
}

#[derive(Debug)]
pub enum Node {
    File { name: String, size: usize },
    Folder { name: String, children: Vec<Node> },
}

impl Node {
    pub fn name(&self) -> &str {
        match self {
            Node::File { name, size: _ } => name,
            Node::Folder { name, children: _ } => name,
        }
    }
    pub fn size(&self) -> usize {
        match self {
            Node::File { name: _, size } => *size,
            Node::Folder { name: _, children } => children.iter().map(Node::size).sum(),
        }
    }
    pub fn filtered_size(&self) -> usize {
        match self {
            Node::File { name: _, size: _ } => 0,
            Node::Folder { name: _, children } => {
                (if self.size() <= 100_000 {
                    self.size()
                } else {
                    0
                }) + children.iter().map(Node::filtered_size).sum::<usize>()
            }
        }
    }

    pub fn add_child(&mut self, path: &Vec<&str>, child: Node) {
        let mut current: &mut Node = self;
        for segment in path {
            match current {
                Self::Folder { name: _, children } => {
                    current = children.iter_mut().find(|n| &n.name() == segment).unwrap()
                }
                _ => panic!("Tried to add child to file {:?}", current),
            }
        }
        match current {
            Node::File { name: _, size: _ } => panic!("Tried to add child to file {:?}", current),
            Node::Folder { name: _, children } => {
                if !children.iter().any(|c| c.name() == child.name()) {
                    children.push(child)
                }
            }
        };
    }
    pub fn find_folders(&self) -> Vec<&Node> {
        match self {
            Node::File { name: _, size: _ } => vec![],
            Node::Folder { name: _, children } => {
                let mut recurse: Vec<&Node> = children
                    .iter()
                    .flat_map(|child| child.find_folders())
                    .collect();
                recurse.push(self);
                recurse
            }
        }
    }
}
