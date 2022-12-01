fn main() {
    let input = include_str!("in.txt");

    println!("max: {}", input
        .split("\n\n")
        .map(|s| s
            .lines()
            .map(|val| val.parse::<i32>().unwrap())
            .sum::<i32>()
        )
        .max().unwrap()
    );
}
