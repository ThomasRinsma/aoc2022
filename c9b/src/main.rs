use itertools::Itertools;

#[derive(Clone,PartialEq,Eq,Hash)]
struct Position {
    x: i32,
    y: i32,
}

// Diagonally next to, or overlapping?
fn is_next_to(a: &Position, b: &Position) -> bool {
    a.x.abs_diff(b.x) <= 1 && a.y.abs_diff(b.y) <= 1
}

// For debugging purposes
fn print_rope(rope: &Vec<Position>) {
    let field_min = Position {
        x: rope.iter().map(|r| r.x).min().unwrap() - 5,
        y: rope.iter().map(|r| r.y).min().unwrap() - 5,
    };

    let field_max = Position {
        x: rope.iter().map(|r| r.x).max().unwrap() + 5,
        y: rope.iter().map(|r| r.y).max().unwrap() + 5,
    };

    for y in (field_min.y..field_max.y).rev() {
        for x in field_min.x..field_max.x {
            // First knot at this position is printed
            let mut has_knot = false;
            for (idx, knot) in rope.iter().enumerate() {
                if knot.x == x && knot.y == y {
                    print!("{}", idx);
                    has_knot = true;
                    break;
                }
            }

            if !has_knot {
                print!(".");
            }

        }
        print!("\n");
    }

    print!("\n\n");
}

fn main() {
    let input = include_str!("in.txt");

    let mut rope = vec![Position {x: 0, y: 0}; 10];
    let mut tail_positions: Vec<Position> = Vec::new();

    for (dir, amount_s) in input.lines().map(|l| l.split_once(' ').unwrap()) {
        // Parse amount
        let amount: i32 = amount_s.parse().unwrap();

        println!("move: {} {}", dir, amount);

        // Move step-by-step
        for _ in 0..amount {
            // Update head position
            match dir {
                "U" => rope[0].y += 1,
                "D" => rope[0].y -= 1,
                "R" => rope[0].x += 1,
                "L" => rope[0].x -= 1,
                _ => ()
            };

            // Move every tail piece if needed
            for i in 1..10 {

                if !is_next_to(&rope[i-1], &rope[i]) {
                    // Move tail in direction of head. Can go diagonal too.
                    if rope[i-1].x > rope[i].x {
                        rope[i].x += 1
                    }
                    else if rope[i-1].x < rope[i].x {
                        rope[i].x -= 1
                    }
    
                    if rope[i-1].y > rope[i].y {
                        rope[i].y += 1
                    }
                    else if rope[i-1].y < rope[i].y {
                        rope[i].y -= 1
                    }
                }
            }

            // Collect location history of final tail piece
            tail_positions.push(rope[9].clone());

            print_rope(&rope);
        }
    }

    let result = tail_positions.iter().unique().count();

    println!("result = {}", result);

}
