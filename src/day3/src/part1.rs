use std::ops::Range;

use regex::Regex;

fn parse_input_to_matrix(input: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    let lines = input.lines();
    for line in lines {
        let mut row = vec![];

        for character in line.chars() {
            row.push(character);
        }

        matrix.push(row)
    }

    return matrix;
}

fn symbols_around(matrix: Vec<Vec<char>>, start_x: usize, end_x: usize, y: usize) -> bool {
    let symbol_regex = Regex::new(r"[^\w\.]").unwrap();

    // Left and right
    if start_x > 0 {
        if let Some(val) = matrix[y].get(start_x-1) {
            //println!("left: {}", val);
            if symbol_regex.is_match(&val.to_string()) {
                return true;
            }
        }
    }
    
    if end_x < matrix[y].len() {
        if let Some(val) = matrix[y].get(end_x+1) {
            //println!("right: {}", val);
            if symbol_regex.is_match(&val.to_string()) {
                return true;
            }
        }
    }

    // Search above and below and +/- 1 on the x axis
    let x_range = Range {
        start: if start_x > 0 {start_x - 1} else {0},
        end: if end_x < matrix[y].len() {end_x + 1} else {0},
    };

    for x in x_range {
        if y > 0 {
            // There is an above :)
            if let Some(val) = matrix[y-1].get(x) {
                //println!("above: {}", val);
                if symbol_regex.is_match(&val.to_string()) {
                    return true;
                }
            }
        }
        if (y + 1) < matrix.len() {
            // There is a below :)
            if let Some(val) = matrix[y+1].get(x) {
                //println!("below: {}", val);
                if symbol_regex.is_match(&val.to_string()) {
                    return true;
                }
            }
        }
    }

    return false;
}

fn run(input: &str) -> usize {
    let mut result: usize = 0;

    let matrix = parse_input_to_matrix(input);

    println!("{:?}", matrix);

    let num_regex = Regex::new(r"[\d]").unwrap();

    for (row_i, row) in matrix.iter().enumerate() {
        let mut row_iter = row.iter();
        let mut col_i = 0;
        let mut col: &char;

        while col_i < row.len() {
            col = row_iter.next().unwrap();
            print!("{}", col);

            if num_regex.is_match(&col.to_string()) {
                let start_x = col_i;
                let mut full_number = format!("{}", col);
                let mut end_x = col_i;
                while let Some(next_char) = row_iter.next() {
                    col_i += 1;
                    print!("{}", next_char);
                    if num_regex.is_match(&next_char.to_string()) {
                        full_number.push(*next_char);
                        end_x += 1;
                    } else {
                        break;
                    }

                }
    
                println!("\nfull number: {}", full_number);
                println!("startx: {}, endx: {}, y: {}", start_x, end_x, row_i);

                if symbols_around(matrix.clone(), start_x, end_x, row_i) {
                    result += full_number.parse::<usize>().unwrap();
                }
            }
            
            col_i += 1;
        }
        print!("\n");
    }

    return result;
}

#[cfg(test)]
mod test {
    use crate::run;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_sample() {
        let input = fs::read_to_string(Path::new("./src/sample.txt")).unwrap();

        let result = run(&input);

        assert_eq!(result, 4361);
    }
}
