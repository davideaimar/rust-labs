

fn count_adjacent(row: usize, col: usize, grid: &[&str], num_rows: usize, num_cols: usize) -> u8 {
    let mut count = 0;

    if row > 0 && col > 0 && grid[row - 1].as_bytes()[col - 1] == b'*' {
        count += 1;
    }
    if row > 0 && grid[row - 1].as_bytes()[col] == b'*' {
        count += 1;
    }
    if row > 0 && col < num_cols - 1 && grid[row - 1].as_bytes()[col + 1] == b'*' {
        count += 1;
    }


    if col > 0 && grid[row].as_bytes()[col - 1] == b'*' {
        count += 1;
    }
    if col < num_cols - 1 && grid[row].as_bytes()[col + 1] == b'*' {
        count += 1;
    }

    if row < num_rows - 1 && col > 0 && grid[row + 1].as_bytes()[col - 1] == b'*' {
        count += 1;
    }
    if row < num_rows - 1 && grid[row + 1].as_bytes()[col] == b'*' {
        count += 1;
    }
    if row < num_rows - 1 && col < num_cols - 1 && grid[row + 1].as_bytes()[col + 1] == b'*' {
        count += 1;
    }

    count
}

/// replace empty spaces with the number of adjacent mines
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    // Loop over rows
    for row in 0..minefield.len() {
        let mut new_row = String::new();
        for col in 0..minefield[row].as_bytes().len(){
            if minefield[row].as_bytes()[col] != b'*' {
                let count = count_adjacent(row, col, minefield, minefield.len(), minefield[row].as_bytes().len());
                match count {
                    0 => new_row.push_str(" "),
                    _ => new_row.push_str(count.to_string().as_str()),
                }
            }
            else{
                new_row.push('*');
            }
        }
        result.push(new_row);
    }
    result
}
