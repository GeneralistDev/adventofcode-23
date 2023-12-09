fn parse_input_to_matrix(input: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    let lines = input.lines();
    for (pos, line) in lines.enumerate() {
        let mut row = vec![];

        for character in line.chars() {
            row.push(character);
        }

        matrix.push(row)
    }

    return matrix;
}

fn run(input: &str) -> usize {
    let mut result: usize = 0;

    let matrix = parse_input_to_matrix(input);

    println!("{:?}", matrix);

    return result;
}

#[cfg(test)]
mod test {
    use crate::run;

    #[test]
    fn test_sample() {
        let input = r#"467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."#;

        let result = run(input);

        assert_eq!(result, 4361);
    }
}