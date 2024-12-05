const XMAS: &str = "XMAS";
const SENTINEL_CHAR: char = '~';

fn part1(input: &str) -> u64 {
    let mut num_xmas = 0;
    let mut word_search: Vec<_> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    // pad word_search
    let word_search_r = word_search.len();
    let word_search_c = word_search[0].len();
    for _ in 0..XMAS.len() {
        word_search.insert(0, vec![SENTINEL_CHAR; word_search_c + 2 * XMAS.len()]);
        word_search.push(vec![SENTINEL_CHAR; word_search_c + 2 * XMAS.len()]);
    }
    let word_search = word_search
        .into_iter()
        .map(|mut x| {
            for _ in 0..XMAS.len() {
                x.insert(0, SENTINEL_CHAR);
                x.push(SENTINEL_CHAR);
            }
            x
        })
        .collect::<Vec<_>>();
    for row_idx in XMAS.len()..XMAS.len() + word_search_r {
        let row = word_search[row_idx].clone();
        // horizontal forwards & backwards check
        for window in row.windows(XMAS.len()) {
            if &window.iter().collect::<String>() == XMAS
                || &window.iter().rev().collect::<String>() == XMAS
            {
                num_xmas += 1;
            }
        }

        // vertical & diagonal check
        let mut vertical_window = [SENTINEL_CHAR; XMAS.len()];
        let mut diagonal_down_window = [SENTINEL_CHAR; XMAS.len()];
        let mut diagonal_up_window = [SENTINEL_CHAR; XMAS.len()];
        for char_idx in XMAS.len()..XMAS.len() + word_search_c {
            for i in 0..XMAS.len() {
                vertical_window[i] = word_search[row_idx + i][char_idx];
            }
            if &vertical_window.iter().collect::<String>() == XMAS
                || &vertical_window.iter().rev().collect::<String>() == XMAS
            {
                num_xmas += 1;
            }
            for i in 0..XMAS.len() {
                diagonal_down_window[i] = word_search[row_idx + i][char_idx + i];
                diagonal_up_window[i] = word_search[row_idx - i][char_idx + i];
            }
            if &diagonal_down_window.iter().collect::<String>() == XMAS
                || &diagonal_down_window.iter().rev().collect::<String>() == XMAS
            {
                num_xmas += 1;
            }
            if &diagonal_up_window.iter().collect::<String>() == XMAS
                || &diagonal_up_window.iter().rev().collect::<String>() == XMAS
            {
                num_xmas += 1;
            }
        }
    }
    num_xmas
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
