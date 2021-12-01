fn main() {
    let mut output = 0;

    let input = include_str!("../../input.txt")
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect::<Vec<i64>>();

    // dbg!(input);
    for i in input.windows(2) {
        if i[1] > i[0] {
            output += 1;
        }
    }
    println!("Output: {}", output);
}
