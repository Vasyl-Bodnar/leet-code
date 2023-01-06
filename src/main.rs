use owo_colors::OwoColorize;
use std::cmp::PartialEq;
use std::fmt::Debug;
// TODO: Move solutions to separate modules, not only to not make this file very large, but also to
// avoid cluttering from owo_colors suggesting me to paint each of my numbers each time I use
// rust_analyzer suggestions

/// 686. Repeated String Match
///
/// Example: `a = "abcd"`, `b = "cdabcdab"`, output = `3`
/// Very Brute Force Solution
/// Repeat a, if it contains b then return the amount of times a was repeated, else continue
/// to avoid running forever, an upper limit is made from dividing lengths of b and a and adding 2
/// on top of that, this does indeed work and solves the problem no worse than anyone's inefficient
/// solution
fn repeated_string_match(a: String, b: String) -> i32 {
    for i in 1..((b.len() as f32 / a.len() as f32).ceil() as usize + 2) {
        if a.repeat(i).contains(&b) {
            return i as i32;
        }
    }
    -1
}

/// 693. Binary Number with Alternating Bits
///
/// Using this simple function with pattern (*2, *2+1, *2, ...)
/// ```
/// |n| -> i32 {
///     [|x:i32| x*2, |x| x*2+1].iter().cycle().take(n).fold(1, |acc,x| x(acc))
/// }
/// ```
/// You can generate any number where the pattern exists, which are always very roughly `2^n`
/// with `n` up to `31` as limited by `i32`, then just match, or cheat like this function does
fn has_alternating_bits(n: i32) -> bool {
    [
        2, 5, 10, 21, 42, 85, 170, 341, 682, 1365, 2730, 5461, 10922, 21845, 43690, 87381, 174762,
        349525, 699050, 1398101, 2796202, 5592405, 11184810, 22369621, 44739242, 89478485,
        178956970, 357913941, 715827882, 1431655765,
    ]
    .contains(&n)
}

macro_rules! print_function_name {
    ($func:expr) => {
        println!("Name: {}", stringify!($func).yellow());
    };
}

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
        $(print_and_check(stringify!($($input)*), $func($($input,)*), $expected);)*
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

/// Store all the test runs from solved problems in main into this function to avoid clutter
#[allow(unused)]
fn solved() {
    group_print!(
        repeated_string_match,
        String::from("a"), String::from("b"); -1,
        String::from("aa"), String::from("a"); 1,
        String::from("a"), String::from("aa"); 2,
        String::from("aaac"), String::from("aac"); 1,
        String::from("abc"), String::from("cabcabca"); 4,
        String::from("abcd"), String::from("cdabcdab"); 3
    );
    group_print!(
        has_alternating_bits,
        5; true, 7; false, 11; false
    );
}

#[test]
fn hab_693() {
    assert!(has_alternating_bits(5));
    assert!(has_alternating_bits(7) == false);
    assert!(has_alternating_bits(11) == false);
}

#[test]
fn rsm_686() {
    assert!(repeated_string_match("a".to_string(), "b".to_string()) == -1);
    assert!(repeated_string_match("aa".to_string(), "a".to_string()) == 1);
    assert!(repeated_string_match("a".to_string(), "aa".to_string()) == 2);
    assert!(repeated_string_match("abcd".to_string(), "cdabcdab".to_string()) == 3);
}
