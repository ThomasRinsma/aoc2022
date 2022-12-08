type DirEntryId = usize;

#[derive(Debug)]
struct DirEntry {
    name: String,
    size: Option<usize>,
    children: Vec<DirEntryId>,
    parent: DirEntryId,
}

fn parse_input_into_tree(input: &str, tree: &mut Vec<DirEntry>) {
    // Parse command-response pairs
    let pairs = input
    .split("$ ")
    .skip(1)
    .map(|c| {
        c.split_once("\n").unwrap()
    });

    tree.push(DirEntry {
        name: String::from("/"),
        size: None,
        children: Vec::new(),
        parent: 0,
    });
    let mut active_entry: DirEntryId = 0;

    for (cmdline, resp) in pairs {
        let (cmd, arg) = cmdline.split_once(' ').unwrap_or((cmdline, ""));
        
        match cmd {
            "cd" => match arg {
                "/" => (), // no-op as this happens only initially
                ".." => {
                    active_entry = tree[active_entry].parent
                },
                _ => {
                    // Find the folder with this name, within the current folder
                    for child in tree[active_entry].children.iter() {
                        if &tree[*child].name == arg {
                            active_entry = *child;
                            break;
                        }
                    }
                }
            }
            "ls" => {
                // Add the listed entries
                for entry in resp.lines() {
                    let (size, name) = entry.split_once(' ').unwrap();
                    tree.push(DirEntry {
                        name: String::from(name),
                        size: match size {
                            "dir" => None,
                            _ => Some(size.parse().unwrap())
                        },
                        children: Vec::new(),
                        parent: active_entry
                    });
                    
                    let idx = tree.len() - 1;
                    tree[active_entry].children.push(idx);
                }
            },
            _ => panic!("invalid")

        }();
    }
}

// Helper to get recursive dir size
fn dir_size(tree: &Vec<DirEntry>, dir: usize, visited_sizes: &mut Vec<usize>) -> usize {
    tree[dir].children
        .iter()
        .map(|c| {
            match tree[*c].size {
                Some(size) => size,
                None => {
                    let size = dir_size(tree, *c, visited_sizes);
                    visited_sizes.push(size);
                    size
                }
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("in.txt");

    let mut tree = Vec::<DirEntry>::new();
    
    parse_input_into_tree(input, &mut tree);

    let mut all_sizes: Vec<usize> = Vec::new();
    
    let used_space = dir_size(&mut tree, 0, &mut all_sizes);
    let free_space = 70_000_000 - used_space;

    let to_delete = 30_000_000 - free_space;

    println!("to_delete = {}", to_delete);

    all_sizes.sort();

    let result: usize = *all_sizes
        .iter()
        .filter(|s| **s >= to_delete)
        .next()
        .unwrap();

    println!("result = {}", result);
}
