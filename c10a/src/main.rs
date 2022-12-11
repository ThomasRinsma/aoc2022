
fn main() {
    let input = include_str!("in.txt");

    let mut x = 1;
    let mut cycles = 0;

    let mut strength_sum = 0;


    for (instr, arg) in input.lines().map(|l| l.split_once(' ').unwrap_or((l, ""))) {
        match instr {
            "noop" => {
                cycles += 1;
                if (cycles - 20) % 40 == 0 {
                    strength_sum += cycles * x;
                }
            }
            "addx" => {
                for _ in 0..2 {
                    cycles += 1;
                    if (cycles - 20) % 40 == 0 {
                        strength_sum += cycles * x;
                    }
                }
                x += arg.parse::<i32>().unwrap()
            },
            _ => ()
        }

    }

    println!("result: {}", strength_sum);
}
