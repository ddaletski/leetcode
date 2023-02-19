pub mod binary_tree;
pub mod disjoint_set;
pub mod linked_list;
pub mod trie;

#[macro_export]
macro_rules! assert_returns {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($ret_value:expr, $func:expr, $($args:expr),*) => {
        // `stringify!` will convert the expression *as it is* into a string.
        let mut error_msg = format!(
            "expected result: {:?},\nfunction: {:?},\nargs:",
            $ret_value,
            stringify!($func),
            );
        $(
            error_msg += format!("\n  {:?}", $args).as_str();
        )*
            error_msg += "\n";

        assert_eq!($func($($args),*), $ret_value, "\n{:}", error_msg);
    };
}

/// 2d vector literal macro
#[macro_export]
macro_rules! vec2d {
    [] => { Vec::new() };

    [$([$($value:expr),*]),*] => {
        vec![$(vec![$($value),*]),*]
    };
}

pub fn parse_2d_array(input_str: &str) -> Vec<Vec<i32>> {
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

#[cfg(test)]
mod tests {
    mod vec2d_tests {
        #[test]
        fn empty_vec() {
            let actual: Vec<Vec<i32>> = vec2d![];
            let expected: Vec<Vec<i32>> = vec![];
            assert_eq!(actual, expected);
        }

        #[test]
        fn non_empty_vec() {
            let expected: Vec<Vec<i32>> = vec![vec![], vec![1], vec![1, 2]];
            assert_eq!(vec2d![[], [1], [1, 2]], expected);
        }
    }

    mod parse_2d_array_tests {
        use crate::parse_2d_array;

        #[test]
        fn case1() {
            let input_str = "[[1,2,3],[4,5,6],[7,8,9]]";
            let expected = vec2d![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
            let parsed = parse_2d_array(input_str);
            assert_eq!(expected, parsed);
        }

        #[test]
        fn case2() {
            let input_str = "[[112,72],[6],[],[70124234,8,9]]";
            let expected = vec2d![[112, 72], [6], [], [70124234, 8, 9]];
            let parsed = parse_2d_array(input_str);
            assert_eq!(parsed, expected);
        }

        #[test]
        fn parser_works_on_empty_array() {
            let input_str = "[]";
            let expected: Vec<Vec<i32>> = vec2d![];
            let parsed = parse_2d_array(input_str);
            assert_eq!(parsed, expected);
        }
    }
}
