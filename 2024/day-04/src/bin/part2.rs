const XMAS: &str = "XMAS";
const SENTINEL_CHAR: char = '~';

const X_MAS_R: usize = 3;
const X_MAS_C: usize = 3;

pub fn part2(input: &str) -> u64 {
    let mut num_xmas = 0;
    let mut word_search: Vec<_> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    // pad word_search
    let word_search_c = word_search[0].len();
    for _ in 0..X_MAS_R {
        word_search.insert(0, vec![SENTINEL_CHAR; word_search_c + 2 * X_MAS_C]);
        word_search.push(vec![SENTINEL_CHAR; word_search_c + 2 * X_MAS_C]);
    }
    let word_search = word_search
        .into_iter()
        .map(|mut x| {
            for _ in 0..X_MAS_C {
                x.insert(0, SENTINEL_CHAR);
                x.push(SENTINEL_CHAR);
            }
            x
        })
        .collect::<Vec<_>>();
    let mut x_mas_window = [[SENTINEL_CHAR; X_MAS_C]; X_MAS_R];
    // window out rows
    for row_window in word_search.windows(X_MAS_R).skip(X_MAS_R) {
        // window out columns
        for col_idx in X_MAS_C..X_MAS_C + word_search_c {
            // load window into array
            for (idx, x_mas_window_row) in row_window.iter().enumerate() {
                x_mas_window[idx] = x_mas_window_row[col_idx..col_idx + X_MAS_C]
                    .try_into()
                    .unwrap();
            }
            if is_xmas_cross(x_mas_window) {
                num_xmas += 1;
            }
        }
    }
    num_xmas
}

fn is_xmas_cross(input: [[char; X_MAS_C]; X_MAS_R]) -> bool {
    let middle = input[1][1];
    let top_left = input[0][0];
    let top_right = input[0][2];
    let bottom_left = input[2][0];
    let bottom_right = input[2][2];
    if middle == 'A' {
        if top_left == 'M' && bottom_left == 'M' {
            return top_right == 'S' && bottom_right == 'S';
        }
        if top_left == 'M' && top_right == 'M' {
            return bottom_left == 'S' && bottom_right == 'S';
        }
        if top_right == 'M' && bottom_right == 'M' {
            return top_left == 'S' && bottom_left == 'S';
        }
        if bottom_left == 'M' && bottom_right == 'M' {
            return top_left == 'S' && top_right == 'S';
        }
    }
    false
}

fn main() {
    let input = include_str!("../../../../advent-of-code-input/2024/day-04.txt");
    let output = part2(input);
    println!("{output}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let output = part2("MMMSXXMASM
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
