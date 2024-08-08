struct Solution;

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let validate = |r: i32, c: i32| r >= 0 && r < rows && c >= 0 && c < cols;
        let next_coord = |r: i32, c: i32, direction: i32| -> (i32, i32) {
            match direction {
                0 => (r, c + 1),
                1 => (r + 1, c),
                2 => (r, c - 1),
                3 => (r - 1, c),
                _ => (0, 0),
            }
        };

        let mut coords: Vec<Vec<i32>> = vec![vec![r_start, c_start]];
        let mut steps_left = 1;
        let mut stride = 1;
        let mut rotates = 0;
        let mut direction = 0; // Right: 0, Down: 1, Left: 2, Up: 3
        let mut cur_r = r_start;
        let mut cur_c = c_start;

        while coords.len() < (rows * cols) as usize {
            (cur_r, cur_c) = next_coord(cur_r, cur_c, direction);
            if validate(cur_r, cur_c) {
                coords.push(vec![cur_r, cur_c]);
            }

            steps_left -= 1;
            if steps_left == 0 {
                direction = (direction + 1) % 4;

                rotates += 1;
                if rotates == 2 {
                    stride += 1;
                    rotates = 0;
                }
                steps_left = stride
            }
        }

        coords
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn array2d_to_vec<const C: usize, const R: usize>(array: &[[i32; C]; R]) -> Vec<Vec<i32>> {
        array.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn test() {
        assert_eq!(
            Solution::spiral_matrix_iii(1, 4, 0, 0),
            array2d_to_vec(&[[0, 0], [0, 1], [0, 2], [0, 3]])
        );
        assert_eq!(
            Solution::spiral_matrix_iii(5, 6, 1, 4),
            array2d_to_vec(&[
                [1, 4],
                [1, 5],
                [2, 5],
                [2, 4],
                [2, 3],
                [1, 3],
                [0, 3],
                [0, 4],
                [0, 5],
                [3, 5],
                [3, 4],
                [3, 3],
                [3, 2],
                [2, 2],
                [1, 2],
                [0, 2],
                [4, 5],
                [4, 4],
                [4, 3],
                [4, 2],
                [4, 1],
                [3, 1],
                [2, 1],
                [1, 1],
                [0, 1],
                [4, 0],
                [3, 0],
                [2, 0],
                [1, 0],
                [0, 0]
            ])
        );
    }
}
