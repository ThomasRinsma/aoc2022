fn main() {
    let input = include_str!("in.txt");

    let (in_stacks, in_moves) = input
        .split_once("\n\n")
        .unwrap();

    // Find the last of the stack numbers (== amount of stacks)
    let num_stacks = in_stacks
        .lines()
        .last().unwrap()
        .trim().split(' ')
        .last().unwrap()
        .parse::<usize>().unwrap();

    // Allocate
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];

    // Take the visual stacks and reverse the lines to build real stacks
    for line in in_stacks.lines().rev().skip(1) {
        // Append values
        for i in 0..num_stacks {
            let c = line.chars().nth(1 + (4 * i)).unwrap();
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    // Process the moves
    for line in in_moves.lines() {
        let words = line.split(' ').collect::<Vec<_>>();

        let amount: usize = words[1].parse().unwrap();
        let from: usize = words[3].parse().unwrap();
        let to: usize = words[5].parse().unwrap();

        for _ in 0..amount {
            let x = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(x);
        }
    }

    let result = stacks.iter().map(|stack| stack.last().unwrap()).collect::<String>();
    println!("result: {}", result);
}
