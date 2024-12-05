fn part1(input: &str) -> u32 {
    let mut occurrences = 0;
    let mut character_array = vec![];
    for row in input.split("\n").into_iter() {
        character_array.push(row.chars().collect::<Vec<char>>());
    }
    let num_rows = character_array.len();
    let num_cols = character_array[0].len();

    for row in 0..num_rows {
        for col in 0..num_cols {
            if is_xmas_here(&character_array, row, col) {
                occurrences += 1;
            }
        }
    }
    return occurrences;
}

fn is_xmas_here(array: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let word = "XMAS".chars().collect::<Vec<_>>();
    if array[row][col] != word[0 as usize] {
        return false;
    }
    let n = array.len();
    let m = array[0].len();


    let x: Vec<i32> = vec![-1, -1, -1, 0, 0, 1, 1, 1];
    let y: Vec<i32> = vec![-1, 0, 1, -1, 1, -1, 0, 1];
    let mut last_index = 0;

    for dir in 0..8 {
        let mut curr_x: i32 = row as i32 + x[dir];
        let mut curr_y: i32 = col as i32 + y[dir];
        for k in 1..word.len() {
            last_index = k;
            if curr_x >= n as i32 || curr_x < 0 || curr_y >= m as i32 || curr_y < 0 {
                break;
            }

            if array[curr_x as usize][curr_y as usize] != word[k] {
                break;
            }
            println!("row {curr_x}, column {curr_y}, letter {}", array[curr_x as usize][curr_y as usize]);
            curr_x += x[dir];
            curr_y += y[dir];
        }
        if last_index == word.len() - 1 {

            return true;
        }
    }
    return false;

}

fn main() {
    let input = include_str!("../../../../advent-of-code-input/2024/day-04.txt");
    let output = part1(input);
    println!("{output}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let output = part1("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");
        assert_eq!(output, 18);
    }
}
