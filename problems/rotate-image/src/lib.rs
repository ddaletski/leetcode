pub struct Solution {}
pub struct Solution2 {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let even = (n % 2 == 0) as usize;

        for radius in 1..=n / 2 {
            for i in 0..(radius * 2 - even) {
                let xy1 = n / 2 - radius;
                let xy2 = n / 2 + radius - even;

                let a = matrix[xy1][xy1 + i];
                let b = matrix[xy1 + i][xy2];
                let c = matrix[xy2][xy2 - i];
                let d = matrix[xy2 - i][xy1];

                println!("i: {}, x1: {}, x2: {}", i, xy1, xy2);

                println!("{}, {}, {}, {}", a, b, c, d);

                matrix[xy1][xy1 + i] = d;
                matrix[xy1 + i][xy2] = a;
                matrix[xy2][xy2 - i] = b;
                matrix[xy2 - i][xy1] = c;
            }
        }
    }
}

impl Solution2 {
    fn flip(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for y in 0..n/2 {
            matrix.swap(y, n-y-1);
        }
    }

    fn transpose(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for y in 0..n {
            for x in 0..y {
                let a = matrix[y][x];
                let b = matrix[x][y];

                matrix[y][x] = b;
                matrix[x][y] = a;
            }
        }
    }

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        Solution2::flip(matrix);
        Solution2::transpose(matrix);
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;
    use crate::Solution2;

    fn parse_2d_array(input_str: &str) -> Vec<Vec<i32>> {
        let mut result = vec![];

        let mut current_vec: Vec<i32> = vec![];

        let mut current_num = 0;
        let mut is_num = false;
        let mut is_arr = false;
        for ch in input_str.chars().skip(1) {
            match ch {
                '[' => {
                    is_arr = true;
                }
                ']' => {
                    if is_arr {
                        is_arr = false;
                        if is_num {
                            is_num = false;
                            current_vec.push(current_num);
                        }
                        current_num = 0;
                        result.push(std::mem::replace(&mut current_vec, vec![]));
                    } else {
                        break;
                    }
                }
                ',' => {
                    if is_arr {
                        is_num = false;
                        current_vec.push(current_num);
                        current_num = 0;
                    }
                }
                digit @ '0'..='9' => {
                    if !is_num {
                        is_num = true;
                        current_num = digit.to_digit(10).unwrap() as i32;
                    } else {
                        current_num = current_num * 10 + digit.to_digit(10).unwrap() as i32;
                    }
                }
                ' ' => {}
                _ => {
                    unreachable!()
                }
            }
        }
        result
    }

    #[test]
    fn parser_test1() {
        let input_str = "[[1,2,3],[4,5,6],[7,8,9]]";
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let parsed = parse_2d_array(input_str);
        assert_eq!(expected, parsed);
    }

    #[test]
    fn parser_test2() {
        let input_str = "[[112,72],[6],[],[70124234,8,9]]";
        let expected = vec![vec![112, 72], vec![6], vec![], vec![70124234, 8, 9]];
        let parsed = parse_2d_array(input_str);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn parser_works_on_empty_array() {
        let input_str = "[]";
        let expected: Vec<Vec<i32>> = vec![];
        let parsed = parse_2d_array(input_str);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test1_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];

        Solution::rotate(&mut matrix);

        assert_eq!(matrix, expected);
    }

    #[test]
    fn test1_2() {
        let input_str = "[[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]";
        let expected_str = "[[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]";

        let mut matrix = parse_2d_array(input_str);
        let expected = parse_2d_array(expected_str);

        Solution::rotate(&mut matrix);

        assert_eq!(matrix, expected);
    }

    #[test]
    fn test2_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];

        Solution2::rotate(&mut matrix);

        assert_eq!(matrix, expected);
    }

    #[test]
    fn test2_2() {
        let input_str = "[[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]";
        let expected_str = "[[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]";

        let mut matrix = parse_2d_array(input_str);
        let expected = parse_2d_array(expected_str);

        Solution2::rotate(&mut matrix);

        assert_eq!(matrix, expected);
    }
}
