fn main() {
    let input = include_str!("in.txt");

    let width = input.find('\n').unwrap();
    let height = input.lines().count();

    // Fill the grid
    let mut grid: Vec<char> = Vec::with_capacity(width * height);
    for line in input.lines() {
        for tree in line.chars() {
            grid.push(tree);
        }
    }

    // Check for every tree
    let mut scenic_scores = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let tree = grid[y * width + x];

            let score: u32 = [
                (0..x).rev().map(|i| grid[y * width + i]).collect(),
                (x + 1..width).map(|i| grid[y * width + i]).collect(),
                (0..y).rev().map(|i| grid[i * width + x]).collect(),
                (y + 1..height).map(|i| grid[i * width + x]).collect(),
            ]
                .iter()
                .map(|range: &Vec<char>| {
                    let mut score = 0;
                    for other in range.iter() {
                        score += (*other < tree) as u32;
                        if *other >= tree {
                            score += 1;
                            break;
                        }
                    }
                    score
                })
                .product();

            scenic_scores.push(score);
        }
    }

    scenic_scores.sort();

    let result = scenic_scores.last().unwrap();
    println!("result = {}", result);
}
