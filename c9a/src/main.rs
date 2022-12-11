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

fn main() {
    let input = include_str!("in.txt");

    let mut head = Position {x: 0, y: 0};
    let mut tail = Position {x: 0, y: 0};

    let mut tail_positions: Vec<Position> = Vec::new();

    for (dir, amount_s) in input.lines().map(|l| l.split_once(' ').unwrap()) {
        // Parse amount
        let amount: i32 = amount_s.parse().unwrap();

        // Move step-by-step
        for _ in 0..amount {
            // Update head position
            match dir {
                "U" => head.y += 1,
                "D" => head.y -= 1,
                "R" => head.x += 1,
                "L" => head.x -= 1,
                _ => ()
            };

            if !is_next_to(&head, &tail) {
                // Move tail in direction of head. Can go diagonal too.
                if head.x > tail.x {
                    tail.x += 1
                }
                else if head.x < tail.x {
                    tail.x -= 1
                }

                if head.y > tail.y {
                    tail.y += 1
                }
                else if head.y < tail.y {
                    tail.y -= 1
                }
            }

            tail_positions.push(tail.clone());
        }
    }

    let result = tail_positions.iter().unique().count();

    println!("result = {}", result);

}
