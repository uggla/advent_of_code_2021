fn main() {
    let input = include_str!("../../input.txt")
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .map(|x| u16::from_str_radix(x, 2))
        .map(Result::unwrap)
        .collect::<Vec<u16>>();

    // dbg!(input);
    let output = run(&input, 12);
    println!("Output: {}", output.0 * output.1);
}

fn run(input: &Vec<u16>, nb_bit: usize) -> (usize, usize) {
    let mut gamma: Vec<&str> = Vec::with_capacity(nb_bit);
    let mut bit: Vec<usize> = Vec::with_capacity(nb_bit);

    for b in (0..nb_bit).rev() {
        // Initialise output vector
        gamma.push("0");
        // contruct a table of bit
        bit.push(2usize.pow(b as u32));
    }

    dbg!(&bit);
    for b in 0..nb_bit {
        let mut n_one = 0;
        for v in input {
            if v & bit[b] as u16 > 0 {
                n_one += 1;
            }
        }

        if n_one > input.len() / 2 {
            gamma[b] = "1"
        } else {
            gamma[b] = "0"
        }
    }

    let epsilon = reverse_bit(gamma.clone());

    dbg!(&gamma);
    dbg!(&epsilon);

    (
        usize::from_str_radix(&gamma.join(""), 2).unwrap(),
        usize::from_str_radix(&epsilon.join(""), 2).unwrap(),
    )
}

fn reverse_bit(bits: Vec<&str>) -> Vec<&str> {
    bits.iter()
        .map(|mut x| {
            if x == &"1" {
                x = &"0";
            } else {
                x = &"1";
            }
            *x
        })
        .collect()
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
    fn reverse_bit_test() {
        assert_eq!(
            reverse_bit(vec!["1", "1", "0", "1"]),
            vec!["0", "0", "1", "0"]
        );
    }

    #[test]
    fn aoc_test() {
        let input: Vec<u16> = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        assert_eq!(run(&input, 5), (22, 9));
        assert_eq!(
            {
                let output = run(&input, 5);
                output.0 * output.1
            },
            198
        );
    }
}
