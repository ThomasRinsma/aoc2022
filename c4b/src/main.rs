use regex::Regex;

fn main() {
    let input = include_str!("in.txt");

    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let result = input
        .lines()
        .filter(|line| {
            let caps = re.captures(line).unwrap();

            // parse as i32
            let p = |x: &str| x.parse::<i32>().unwrap();

            let res =
                (p(&caps[1]) <= p(&caps[3]) && p(&caps[2]) >= p(&caps[3]))
            ||  (p(&caps[3]) <= p(&caps[1]) && p(&caps[4]) >= p(&caps[1]));


            return res;
        })
        .count();

    println!("result: {}", result);
}
