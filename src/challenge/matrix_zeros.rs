use std::collections;

pub fn make_zeroes(matrix: &mut Vec<Vec<u32>>) {
    let mut reseted_columns = collections::HashSet::<usize>::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if !reseted_columns.contains(&j) && matrix[i][j] == 0 {
                reset_row(i, matrix);
                reset_column(j, matrix);
                reseted_columns.insert(j);
                break;
            }
        }
    }
}

pub fn reset_row(row_index: usize, matrix: &mut Vec<Vec<u32>>) {
    let row = &mut matrix[row_index];
    for v in row {
        *v = 0;
    }
}

pub fn reset_column(col_index: usize, matrix: &mut Vec<Vec<u32>>) {
    for row in matrix {
        row[col_index] = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_validate_example1() {
        // GIVEN
        let mut matrix = vec![
            vec![5, 4, 3, 9],
            vec![2, 0, 7, 6],
            vec![1, 3, 4, 0],
            vec![9, 8, 3, 4],
        ];

        // WHEN
        make_zeroes(&mut matrix);

        // THEN
        assert_eq!(
            matrix,
            vec![
                vec![5, 0, 3, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![9, 0, 3, 0],
            ]
        )
    }
}
