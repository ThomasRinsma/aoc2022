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

    // Check for every tree inside the perimeter
    let mut visible_count = width * 2 + height * 2 - 4; // edges are always visible
    for y in 1..height-1 {
        for x in 1..width-1 {
            let tree = grid[y * width + x];

            // Line to either of the four cardinal directions should be clear
            if (0..x).map(|i| grid[y * width + i]).all(|t| t < tree)
            || (x+1..width).map(|i| grid[y * width + i]).all(|t| t < tree)
            || (0..y).map(|i| grid[i * width + x]).all(|t| t < tree)
            || (y+1..height).map(|i| grid[i * width + x]).all(|t| t < tree)
            {
                visible_count += 1;
            }
        }
    }

    println!("result = {}", visible_count);
}
