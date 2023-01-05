// WIP
/// 686. Repeated String Match
///
/// Try brute force for now
fn repeated_string_match(a: String, b: String) -> i32 {
    todo!()
}

/// 693. Binary Number with Alternating Bits
///
/// Using this simple function
/// `|n| -> i32 [|x:i32| x*2, |x:i32| x*2+1].iter().cycle().take(n).fold(1, |acc,x| x(acc))`
/// you can generate any number where the pattern exists, which are always roughly 2^n
/// with n up to 31 as limited by i32
/// then just match, or cheat like this function does
fn has_alternating_bits(n: i32) -> bool {
    [
        2, 5, 10, 21, 42, 85, 170, 341, 682, 1365, 2730, 5461, 10922, 21845, 43690, 87381, 174762,
        349525, 699050, 1398101, 2796202, 5592405, 11184810, 22369621, 44739242, 89478485,
        178956970, 357913941, 715827882, 1431655765,
    ]
    .contains(&n)
}

fn group_print<I, R, F>(func: F, input: Vec<I>, expected: Vec<R>)
where
    I: std::fmt::Debug + std::cmp::PartialEq + Clone,
    R: std::fmt::Debug + std::cmp::PartialEq,
    F: Fn(I) -> R,
{
    for (inp, exp) in input.into_iter().zip(expected.into_iter()) {
        println!(" ");
        print_and_check(func(inp.clone()), exp, Some(inp));
    }
}

fn print_and_check<
    T: std::fmt::Debug + std::cmp::PartialEq,
    Y: std::fmt::Debug + std::cmp::PartialEq,
>(
    output: T,
    expected: T,
    input: Option<Y>,
) {
    if let Some(input) = input {
        println!("Input:{input:?}");
    }
    println!("Output:{output:?}");
    println!("Expected:{expected:?}");
    println!("Success? {}", if output == expected { "yes" } else { "no" })
}

fn main() {
    /* print_and_check(repeated_string_match("a".to_string(), "aa".to_string()), 2);
    print_and_check(repeated_string_match("abcd".to_string(), "cdabcdab".to_string()), 3); */

    group_print(
        has_alternating_bits,
        vec![5, 7, 11],
        vec![true, false, false],
    )
}

#[test]
fn alternating_bits() {
    assert!(has_alternating_bits(5));
    assert!(has_alternating_bits(7) == false);
    assert!(has_alternating_bits(11) == false);
}
