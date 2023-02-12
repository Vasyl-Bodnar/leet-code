#![allow(unused)]
use owo_colors::OwoColorize;
use std::cmp::PartialEq;
use std::fmt::Debug;

mod solutions;
use solutions::util::*;
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
/// does not matter, it can be `and apply func`, which copies a function onto each input, or `and
/// apply fully func`, which copies onto both input and output.
///
/// As an example, it is useful for cases where input is a string and `String::from` on each element is distracting,
/// or with horrible types like in Problem 2.
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
    ($func:expr, and apply fully $app:expr, $(
        $($input:expr),+;
        $expected:expr
    ),*) => {
        print_function_name!($func);
        $(print_and_check(stringify!($($input)+), $func($($app($input),)+), $app($expected));)*
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
    ($func:expr, and apply fully $app:expr, $(
        $($input:expr),+;
        $expected:expr
    ),*) => {
        $(assert!($func($($app($input),)*) == $app($expected));)*
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
        "Input: {} | Output: {:?} | Expected: {:?} | Success? {}",
        input.blue(),
        output.green(),
        expected.red(),
        if output == expected { "yes" } else { "no" }.magenta()
    )
}

fn main() {}

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
fn atn_2() {
    let f = |x: Vec<i32>| Some(Box::new(ListNode::from_vec(x)));
    group_test!(
        add_two_numbers, and apply fully f,
        vec![2,4,3], vec![5,6,4]; vec![7,0,8],
        vec![0], vec![0]; vec![0],
        vec![9,9,9,9,9,9,9], vec![9,9,9,9]; vec![8,9,9,9,0,0,0,1]
    );
}

#[test]
fn im_10() {
    group_test!(
        is_match, and apply String::from,
        "mississippi", "mis*is*p*."; false,
        "mississippi", "mis*is*ip*."; true,
        "aab", "c*a*b"; true,
        "aa", "a*"; true,
        "aa", "a"; false,
        "aba", ".*"; true,
        "ab", ".*c"; false,
        "aaba", "ab*a*c*a"; false,
        "aaa", "ab*a"; false,
        "aaa", "a*a"; true
    );
}

#[test]
fn itr_12() {
    group_test!(
        int_to_roman,
        3; String::from("III"),
        58; String::from("LVIII"),
        1994; String::from("MCMXCIV")
    );
}

#[test]
fn rti_13() {
    group_test!(
        roman_to_int, and apply String::from,
        "III"; 3,
        "LVIII"; 58,
        "MCMXCIV"; 1994
    );
}

#[test]
fn lcp_14() {
    let f = |vec: Vec<&str>| vec.into_iter().map(String::from).collect();
    group_test!(
        longest_common_prefix, and apply f,
        vec!["flower", "flow", "flight"]; String::from("fl"),
        vec!["dog", "racecar", "car"]; String::from("")
    );
}

#[test]
fn lvp_32() {
    group_test!(
        longest_valid_parentheses, and apply String::from,
        "()(())"; 6,
        "(()"; 2,
        ")()())"; 4,
        ""; 0
    );
}

#[test]
fn cas_38() {
    group_print!(
        count_and_say,
        1; String::from("1"),
        4; String::from("1211")
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
fn ctt_168() {
    group_test!(
        convert_to_title,
        1; String::from("A"),
        28; String::from("AB"),
        701; String::from("ZY")
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
fn vma_941() {
    group_test!(
        valid_mountain_array,
        vec![2,1]; false,
        vec![3,5,5]; false,
        vec![0,3,2,1]; true
    );
}

#[test]
fn bc_991() {
    group_test!(
        broken_calc,
        2, 3; 2,
        5, 8; 2,
        3, 10; 3,
        1, 1_000_000_000; 39
    );
}

#[test]
fn re_1299() {
    group_test!(
        replace_elements,
        vec![17,18,5,4,6,1]; vec![18,6,6,6,1,-1],
        vec![400]; vec![-1]
    );
}

#[test]
fn sz_1304() {
    group_test!(
        sum_zero,
        5; vec![-2,-1,0,1,2], // Many Possible combinations, but this is the simplest required one
        3; vec![-1,0,1],
        1; vec![0]
    );
}

#[test]
fn ipow_1455() {
    group_test!(
        is_prefix_of_word, and apply String::from,
        "i love eating burger", "burg"; 4,
        "this problem is an easy problem", "pro"; 2,
        "i am tired", "you"; -1
    );
}

#[test]
fn fs_1636() {
    group_test!(
        frequency_sort,
        vec![1,1,2,2,2,3]; vec![3,1,1,2,2,2],
        vec![2,3,1,3,2]; vec![1,3,3,2,2],
        vec![-1,1,-6,4,5,-6,1,4,1]; vec![5,-1,4,4,-6,-6,1,1,1]
    );
}

#[test]
fn i_1678() {
    group_test!(
        interpret, and apply fully String::from,
        "G()(al)"; "Goal",
        "G()()()()(al)"; "Gooooal",
        "(al)G(al)()()G"; "alGalooG"
    );
}

#[test]
fn md_2016() {
    group_test!(
        maximum_difference,
        vec![87,68,91,86,58,63,43,98,6,40]; 55,
        vec![7,1,5,4]; 4,
        vec![9,4,3,2]; -1,
        vec![1,5,2,10]; 9
    );
}

#[test]
fn pbc_2055() {
    group_test!(
        plates_between_candles,
        String::from("**|**|***|"), vec![vec![2,5],vec![5,9]]; vec![2,3],
        String::from("***|**|*****|**||**|*"), vec![vec![1,17],vec![4,5],vec![14,17],vec![5,11],vec![15,16]]; vec![9,0,0,0,0]
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
fn da_2207() {
    group_test!(
        divide_array,
        vec![3,2,3,2,2,2]; true,
        vec![1,2,3,4]; false
    );
}

#[test]
fn mfe_2404() {
    group_test!(
        most_frequent_even,
        vec![0,1,2,2,4,4,1]; 2,
        vec![4,4,4,9,2,4]; 4,
        vec![29,47,21,41,13,37,25,7]; -1
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
