#![allow(unused)]
use owo_colors::OwoColorize;
use std::cmp::PartialEq;
use std::fmt::Debug;

mod solutions;
use solutions::*;

macro_rules! print_function_name {
    ($func:expr) => {
        println!("Name: {}", stringify!($func).yellow());
    };
}

/// Simple print macro for functions based around `print_and_check` function
/// Its format is such:
/// ```
/// group_print!(
///     function_name, [and sort,] // Option for solutions where order does not matter
///     [input, input, etc.;] output, // Could be single or multiple inputs or none
///     [input, input, etc.;] output,
///     ...
///     [input, input, etc.;] output
/// )
/// ```
///
/// `group_test` is equivalent in everything except that it focuses around asserts instead
macro_rules! group_print {
    ($func:expr, $expected:expr) =>
    {
        print_function_name!($func);
        print_and_check("None".to_string, $func(), $expected);
    };
    ($func:expr, $(
        $($input:expr),+;
        $expected:expr
    ),*) => {
        print_function_name!($func);
        $(print_and_check(stringify!($($input)+), $func($($input,)+), $expected);)*
    };
    ($func:expr, and sort, $(
        $($input:expr),+;
        $expected:expr
    ),*) => {
        print_function_name!($func);
        $(let mut output = $func($($input,)+);
        let mut expected = $expected;
        output.sort(); expected.sort();
        print_and_check(stringify!($($input)+), output, expected);)*
    };
}

macro_rules! group_test {
    ($func:expr, $expected:expr) =>
    {
        assert!($func() == $expected);
    };
    ($func:expr, $(
        $($input:expr),+;
        $expected:expr
    ),*) => {
        $(assert!($func($($input,)*) == $expected);)*
    };
    ($func:expr, and sort, $(
        $($input:expr),+;
        $expected:expr
    ),*) => {
        $(let mut output = $func($($input,)+);
        let mut expected = $expected;
        output.sort(); expected.sort();
        assert!(output == expected);)*
    };
}

fn print_and_check<T: Debug + PartialEq>(input: &str, output: T, expected: T) {
    println!(
        "Input: {:?} | Output: {:?} | Expected: {:?} | Success? {}",
        input.blue(),
        output.green(),
        expected.red(),
        if output == expected { "yes" } else { "no" }.magenta()
    )
}

fn main() {}

// Solved problems become tests
// Due to group_print and group_test macros being essentially the same,
// you can just place them back into the main,
// change the _test to _print, and test them again
#[test]
fn ts_1() {
    group_test!(
        two_sum, and sort,
        vec![2,7,11,15], 9; vec![0,1],
        vec![3,2,4], 6; vec![1,2],
        vec![3,3], 6; vec![0,1]
    );
}

#[test]
fn rsm_686() {
    group_test!(
        repeated_string_match,
        String::from("a"), String::from("b"); -1,
        String::from("aa"), String::from("a"); 1,
        String::from("a"), String::from("aa"); 2,
        String::from("aaac"), String::from("aac"); 1,
        String::from("abc"), String::from("cabcabca"); 4,
        String::from("abcd"), String::from("cdabcdab"); 3
    );
}

#[test]
fn hab_693() {
    group_test!(
        has_alternating_bits,
        5; true,
        7; false,
        11; false
    );
}

#[test]
fn bc_991() {
    group_print!(
        broken_calc,
        2, 3; 2,
        5, 8; 2,
        3, 10; 3,
        1, 1_000_000_000; 39
    );
}

#[test]
fn pi_2485() {
    group_test!(
        pivot_integer,
        8; 6,
        1; 1,
        4; -1
    );
}
