use std::{cmp::Reverse, collections::HashMap};
use priority_queue::PriorityQueue;

fn print_map(heightmap: &Vec<u32>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            print!("{}", char::from_u32(heightmap[y * width + x]).unwrap());
        }
        print!("\n");
    }
}

// https://en.wikipedia.org/wiki/A*_search_algorithm
fn astar(heightmap: &mut Vec<u32>, width: usize, height: usize, start_pos: usize, end_pos: usize) -> usize {
    let mut queue = PriorityQueue::new();
    queue.push(start_pos, Reverse(0));

    // Length of the shortest path from start_pos
    let mut g_score = vec![std::usize::MAX; width * height];
    g_score[start_pos] = 0;

    // Hashmap to reconstruct path
    let mut came_from: HashMap<usize, usize> = HashMap::new();

    // Helper to get cell neighbors
    let neighbors = |x: usize| {
        let mut result = Vec::new();
        if x % width > 0 && heightmap[x - 1] <= heightmap[x] + 1 {
            result.push(x - 1);
        }
        if x % width < width - 1 && heightmap[x + 1] <= heightmap[x] + 1 {
            result.push(x + 1);
        }
        if x / width > 0 && heightmap[x - width] <= heightmap[x] + 1 {
            result.push(x - width);
        }
        if x / width < height - 1 && heightmap[x + width] <= heightmap[x] + 1 {
            result.push(x + width);
        }
        result
    };

    // Manhattan distance
    let dist = |a: usize, b: usize| {
        (a % width).abs_diff(b % width) + (a / width).abs_diff(b / width)
    };

    // Main A* loop
    while !queue.is_empty() {
        let current = queue.pop().unwrap().0;
        if current == end_pos {
            println!("Finished!");
            // Reconstruct the path from end to start
            let mut path_iter = end_pos;
            let mut len = 0;
            while path_iter != start_pos {
                let prev = came_from[&path_iter];
                len += 1;
                
                // Draw on the map!
                heightmap[path_iter] = 'X' as u32;

                path_iter = prev;
            }

            print_map(heightmap, width, height);

            return len;
        }

        for neighbor in neighbors(current) {
            // Neighbor distance is always 1
            let tenative_g_score = g_score[current] + 1;
            if tenative_g_score < g_score[neighbor] {
                // A new best path to `neighbor`
                came_from.insert(neighbor, current);
                g_score[neighbor] = tenative_g_score;
                let f_score = tenative_g_score + dist(neighbor, end_pos);
                queue.push(neighbor, Reverse(f_score));
            }
        }
    }

    return 0; // No path found
}

fn main() {
    let input = include_str!("in.txt");

    let width = input.find('\n').unwrap();
    let height = input.lines().count();

    // Fill the heightmap
    let mut heightmap: Vec<u32> = Vec::with_capacity(width * height);
    for line in input.lines() {
        for val in line.chars() {
            heightmap.push(val as u32);
        }
    }

    // Save start and end positions
    let start_pos = heightmap.iter().position(|x| *x == 'S' as u32).unwrap();
    let end_pos = heightmap.iter().position(|x| *x == 'E' as u32).unwrap();

    // Then convert to their height values
    heightmap[start_pos] = 'a' as u32;
    heightmap[end_pos] = 'z' as u32;

    // Recursive DFS
    let result = astar(&mut heightmap, width, height, start_pos, end_pos);
    println!("result = {}", result);
}
