
fn main() {
    let input = include_str!("in.txt");

    let mut x = 1;
    let (mut crt_x, mut crt_y) = (0, 0);

    for (instr, arg) in input.lines().map(|l| l.split_once(' ').unwrap_or((l, "0"))) {
        let instr_cycles = match instr {
            "noop" => 1,
            "addx" => 2,
            _ => panic!()
        };

        for _ in 0..instr_cycles {
            // Draw
            if crt_x >= x-1 && crt_x <= x+1 {
                print!("#");
            } else {
                print!(".");
            }
            
            // Move beam around
            crt_x += 1;
            if crt_x == 40 {
                // New line
                print!("\n");
                crt_x = 0;
                crt_y += 1;
            }
            if crt_y == 6 {
                // New frame
                print!("\n\n");
                crt_x = 0;
                crt_y = 0;
            }

        }
        x += arg.parse::<i32>().unwrap();

    }

}
