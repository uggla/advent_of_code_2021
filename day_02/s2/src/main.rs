fn main() {
    let mut depth = 0;
    let mut horiz = 0;
    let mut aim = 0;

    let input = include_str!("../../input.txt")
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .map(|x| {
            let v: Vec<&str> = x.split(' ').collect();
            let out: (&str, i64) = (v[0], v[1].parse().unwrap());
            out
        })
        .collect::<Vec<(&str, i64)>>();

    // dbg!(input);
    run(&input, &mut depth, &mut horiz, &mut aim);
    println!("Output: {}", depth * horiz);
}

fn run(input: &Vec<(&str, i64)>, depth: &mut i64, horiz: &mut i64, aim: &mut i64) {
    for item in input {
        match item.0 {
            "forward" => {
                *horiz += item.1;
                *depth += item.1 * *aim;
            }
            "up" => {
                // *depth -= item.1;
                *aim -= item.1;
            }
            "down" => {
                // *depth += item.1;
                *aim += item.1;
            }
            _ => panic!("Unknown value"),
        }
        dbg!(&horiz);
        dbg!(&depth);
        dbg!(&aim);
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fake() {
        assert_eq!(1, 1);
    }

    #[test]
    fn aoc_test() {
        let mut depth = 0;
        let mut horiz = 0;
        let mut aim = 0;
        let input = vec![
            ("forward", 5),
            ("down", 5),
            ("forward", 8),
            ("up", 3),
            ("down", 8),
            ("forward", 2),
        ];

        run(&input, &mut depth, &mut horiz, &mut aim);
        assert_eq!(horiz, 15);
        assert_eq!(depth, 60);
        assert_eq!(depth * horiz, 900);
    }
}
