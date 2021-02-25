pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let height = grid.len();
    let width = grid[0].len();

    let mut number_of_islands = 0;
    for row in 0..height {
        for col in 0..width {
            if discover_island(&mut grid, row, col, height, width) {
                number_of_islands += 1;
            }
        }
    }

    number_of_islands
}

fn discover_island(
    grid: &mut Vec<Vec<char>>,
    row: usize,
    col: usize,
    height: usize,
    width: usize,
) -> bool {
    if grid[row][col] != '1' {
        return false;
    }

    // clear the island piece, we already discovered it
    grid[row][col] = '0';
    // then try to see in all direction if the island still exists
    if row > 0 {
        // NORTH
        discover_island(grid, row - 1, col, height, width);
    }
    if col + 1 < width {
        // EAST
        discover_island(grid, row, col + 1, height, width);
    }
    if row + 1 < height {
        // SOUTH
        discover_island(grid, row + 1, col, height, width);
    }
    if col > 0 {
        // WEST
        discover_island(grid, row, col - 1, height, width);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_validate_example1() {
        // GIVEN
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];

        // WHEN
        let number_of_islands = num_islands(grid);

        // THEN
        assert_eq!(number_of_islands, 1);
    }

    #[test]
    fn it_should_validate_example2() {
        // GIVEN
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        // WHEN
        let number_of_islands = num_islands(grid);

        // THEN
        assert_eq!(number_of_islands, 3);
    }
}
