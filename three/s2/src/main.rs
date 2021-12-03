fn main() {
    let input = include_str!("../../input.txt")
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .map(|x| u16::from_str_radix(x, 2))
        .map(Result::unwrap)
        .collect::<Vec<u16>>();

    let gamma_and_epsilon = run(&input, 12);
    println!("Output: {}", gamma_and_epsilon.0 * gamma_and_epsilon.1);
    let ogr = filter_input(
        &input,
        bin2vec(gamma_and_epsilon.0, 12),
        LifeSupportRating::Ogr,
    );
    let csr = filter_input(
        &input,
        bin2vec(gamma_and_epsilon.1, 12),
        LifeSupportRating::Csr,
    );
    let result: u64 = ogr as u64 * csr as u64;
    println!("Output: {}", result);
}

fn run(input: &Vec<u16>, nb_bit: usize) -> (usize, usize) {
    let mut gamma: Vec<&str> = Vec::with_capacity(nb_bit);
    let mut bit: Vec<usize> = Vec::with_capacity(nb_bit);

    for bit_index in (0..nb_bit).rev() {
        // Initialise output vector
        gamma.push("0");
        // contruct a table of bit
        bit.push(2usize.pow(bit_index as u32));
    }

    for bit_index in 0..nb_bit {
        let mut n_one = 0;
        for item in input {
            if item & bit[bit_index] as u16 > 0 {
                n_one += 1;
            }
        }

        if n_one > input.len() / 2 {
            gamma[bit_index] = "1"
        } else {
            gamma[bit_index] = "0"
        }
    }

    let epsilon = reverse_bit(gamma.clone());

    (
        usize::from_str_radix(&gamma.join(""), 2).unwrap(),
        usize::from_str_radix(&epsilon.join(""), 2).unwrap(),
    )
}

fn reverse_bit(bits: Vec<&str>) -> Vec<&str> {
    bits.iter()
        .map(|mut bit| {
            if bit == &"1" {
                bit = &"0";
            } else {
                bit = &"1";
            }
            *bit
        })
        .collect()
}

fn bin2vec(value: usize, nb_bit: usize) -> Vec<usize> {
    let output = format!("{:064b}", value);
    let output = &output[(64 - nb_bit)..];
    let output: Vec<usize> = output
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();
    output
}

enum LifeSupportRating {
    Ogr,
    Csr,
}

fn filter_input(input: &Vec<u16>, value: Vec<usize>, kind: LifeSupportRating) -> u16 {
    let mut output = input.clone();
    let mut bit: Vec<usize> = Vec::with_capacity(value.len());
    for bit_index in (0..value.len()).rev() {
        // Initialise bit vector
        bit.push(2usize.pow(bit_index as u32));
    }
    dbg!(&input);
    dbg!(&value);

    let mut bit_index = 0;
    while output.len() > 1 {
        // Determine number of one and zero
        let mut n_one = 0;
        let mut n_zero = 0;
        for item in &output {
            if item & bit[bit_index] as u16 > 0 {
                n_one += 1;
            } else {
                n_zero += 1;
            }
        }

        dbg!(n_one);
        dbg!(n_zero);
        match kind {
            LifeSupportRating::Ogr => {
                if n_one == n_zero {
                    output.retain(|&x| x & bit[bit_index] as u16 > 0);
                } else if n_one > n_zero {
                    output.retain(|&x| {
                        dbg!(x);
                        dbg!(&bit[bit_index]);
                        dbg!(x & bit[bit_index] as u16);
                        dbg!(value[bit_index]);
                        (x & bit[bit_index] as u16) > 0
                    });
                } else {
                    output.retain(|&x| {
                        dbg!(x);
                        dbg!(&bit[bit_index]);
                        dbg!(x & bit[bit_index] as u16);
                        dbg!(value[bit_index]);
                        (x & bit[bit_index] as u16) == 0
                    });
                }
            }
            LifeSupportRating::Csr => {
                if n_one == n_zero {
                    output.retain(|&x| x & bit[bit_index] as u16 == 0);
                } else if n_one > n_zero {
                    output.retain(|&x| {
                        dbg!(x);
                        dbg!(&bit[bit_index]);
                        dbg!(x & bit[bit_index] as u16);
                        dbg!(value[bit_index]);
                        (x & bit[bit_index] as u16) == 0
                    });
                } else {
                    output.retain(|&x| {
                        dbg!(x);
                        dbg!(&bit[bit_index]);
                        dbg!(x & bit[bit_index] as u16);
                        dbg!(value[bit_index]);
                        (x & bit[bit_index] as u16) > 0
                    });
                }
            }
        }
        // Show current output
        dbg!(&output);
        // Next bit
        bit_index += 1;
    }
    output[0]
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
    fn bin2vec_test() {
        assert_eq!(bin2vec(4, 4), vec![0, 1, 0, 0]);
    }

    #[test]
    fn aoc_2_test() {
        let input: Vec<u16> = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        assert_eq!(
            filter_input(&input, bin2vec(22, 5), LifeSupportRating::Ogr),
            23
        );
        assert_eq!(
            filter_input(&input, bin2vec(9, 5), LifeSupportRating::Csr),
            10
        );
    }

    #[test]
    fn aoc_1_test() {
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
