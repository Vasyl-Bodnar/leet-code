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
///
/// Its format is such:
/// ```
/// group_print!(
///     function_name, [option,]
///     [input, input, etc.;] output, // Could be none, single, or multiple inputs
///     [input, input, etc.;] output,
///     ...
///     [input, input, etc.;] output
/// )
/// ```
/// Option can be `and sort`, which sorts the ouput and expected output for problems where order
/// does not matter, or it can be `and apply func`, which copies a function onto each element.
/// As an example, it is useful for cases where input is a string and String::from on each element is distracting.
///
/// `group_test` is equivalent in everything except that it focuses around asserts instead of printing.
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
    ($func:expr, and apply $app:expr, $(
        $($input:expr),+;
        $expected:expr
    ),*) => {
        print_function_name!($func);
        $(print_and_check(stringify!($($input)+), $func($($app($input),)+), $expected);)*
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
    ($func:expr, and apply $app:expr, $(
        $($input:expr),+;
        $expected:expr
    ),*) => {
        $(assert!($func($($app($input),)*) == $expected);)*
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

fn main() {
    group_print!(
        maximum_difference,
        vec![7,1,5,4]; 4,
        vec![9,4,3,2]; -1,
        vec![1,5,2,10]; 9
    );
}

// Solved problems become tests
// Due to group_print and group_test macros being essentially the same,
// they can be nicely used interchangeably
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
fn fm_153() {
    group_test!(
        find_min,
        vec![3,4,5,1,2]; 1,
        vec![4,5,6,7,0,1,2]; 0,
        vec![11,13,15,17]; 11,
        vec![5,1,2,3,4]; 1,
        vec![2,1]; 1
    );
}

#[test]
fn rsm_686() {
    group_test!(
        repeated_string_match, and apply String::from,
        "a", "b"; -1,
        "aa", "a"; 1,
        "a", "aa"; 2,
        "aaac", "aac"; 1,
        "abc", "cabcabca"; 4,
        "abcd", "cdabcdab"; 3
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
fn md_2078() {
    group_test!(
        max_distance,
        vec![4,4,4,11,4,4,11,4,4,4,4,4]; 8,
        vec![1,1,1,6,1,1,1]; 3,
        vec![1,8,3,8,3]; 4,
        vec![82,40,40,82]; 2,
        vec![0,1]; 1
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
