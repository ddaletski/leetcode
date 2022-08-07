pub mod binary_tree;
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
