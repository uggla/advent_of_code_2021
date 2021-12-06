fn main() {
    let input = include_str!("../../input.txt")
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        //.map(|x| u16::from_str_radix(x, 2))
        //.map(Result::unwrap)
        .collect::<Vec<&str>>();

    dbg!(input);
    // let output = run(&input, 12);
    // println!("Output: {}", output.0 * output.1);
}

fn parse_input(input: &Vec<String>) -> Vec<String> {
    todo!()
}

#[derive(Debug, PartialEq)]
struct Board {
    position: Vec<Vec<(usize, bool)>>,
}

impl Board {
    pub fn new() -> Self {
        Self { position: vec![] }
    }

    pub fn parse_row(&mut self, row: &str) {
        let mut board_row: Vec<(usize, bool)> = Vec::new();
        for item in row.split(",") {
            let value: usize = item.parse().unwrap();
            board_row.push((value, false));
        }
        self.position.push(board_row);

        dbg!(row);
    }
}

fn run(input: &Vec<u16>, nb_bit: usize) -> (usize, usize) {
    todo!();
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
        todo!();
    }

    #[test]
    fn parse_row_test() {
        let mut board = Board::new();
        board.parse_row("1,2,3");
        assert_eq!(
            board,
            Board {
                position: vec![vec![(1, false), (2, false), (3, false)]],
            },
        );
    }

    #[test]
    fn check_winning_row_test() {
        todo!();
    }

    #[test]
    fn check_winning_column_test() {
        todo!();
    }

    #[test]
    fn set_position_test() {
        todo!();
    }
}
