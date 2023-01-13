#![allow(unused)]
use std::collections::HashMap;

/// 1. Two Sum - `Easy`
///
/// # Praise
/// A Legendary and Simple Question with just as Simple Solution.
///
/// # Solution
/// Just use `HashMap`.
///
/// `unreachable!` may look a bit off, but since the solution is guaranteed,
/// and no set undefined answer is given (like empty vector or -1)
/// this is the only right answer to the problem in that case.
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash = HashMap::<i32, i32>::new();
    for (i, x) in nums.into_iter().enumerate() {
        if let Some(val) = hash.get(&(target - x)) {
            return vec![i as i32, *val];
        }
        hash.insert(x, i as i32);
    }
    unreachable!()
}

/// 153. Find Minimum in Rotated Sorted Array - `Medium`
///
/// # Idea
/// ```
/// nums.into_iter().min().unwrap()
/// ```
/// For a serious solution, we just need to implement a `Binary Search`.
/// Since everything is sorted, we just need to add a couple of checks
/// for finding the smallest one.
///
/// # Conclusion
/// The solution is indeed a binary search, I used a recursive solution, though
/// the one that is usually used is while loop based, with little difference in perfomance.
pub fn find_min(nums: Vec<i32>) -> i32 {
    let (i, l) = (0, nums.len() - 1);
    let p = l / 2;

    if p == i {
        return if nums[i] < nums[l] { nums[i] } else { nums[l] };
    }

    if nums[l] > nums[i] || nums[p] < nums[i] {
        find_min(nums[i..=p].to_vec())
    } else {
        find_min(nums[p..=l].to_vec())
    }
}

/// 686. Repeated String Match - `Medium`
///
/// # Idea
/// Very Brute Force Solution.
///
/// Repeat `a`, if it contains `b` then return the amount of times a was repeated, else continue
/// to avoid running forever, an upper limit is made from dividing lengths of `b` and `a` and adding 2
/// on top of that.
///
/// This should indeed work and solve the problem no worse than anyone's inefficient solution.
///
/// # Conclusion
/// Though it is not great and not a good answer even among similar solutions,
/// which usually involve concats, this is a fine solution that works quite well,
/// so it is more than good enough for the task.
pub fn repeated_string_match(a: String, b: String) -> i32 {
    for i in 1..((b.len() as f32 / a.len() as f32).ceil() as usize + 2) {
        if a.repeat(i).contains(&b) {
            return i as i32;
        }
    }
    -1
}

/// 693. Binary Number with Alternating Bits - `Easy`
///
/// # Idea
/// Using this simple function with pattern (*2, *2+1, *2, ...).
/// ```
/// |n| -> i32 {
///     [|x:i32| x*2, |x| x*2+1].iter().cycle().take(n).fold(1, |acc,x| x(acc))
/// }
/// ```
/// You can generate any number where the pattern exists, which are always very roughly `2^n`
/// with `n` up to `31` as limited by `i32`, then just match, or cheat like this function does.
///
/// # Conclusion
/// My solution is as efficient for the task as possible, and is such is a perfect answer, though
/// in case if it is considered cheating, then the generator works well enough.
pub fn has_alternating_bits(n: i32) -> bool {
    [
        2, 5, 10, 21, 42, 85, 170, 341, 682, 1365, 2730, 5461, 10922, 21845, 43690, 87381, 174762,
        349525, 699050, 1398101, 2796202, 5592405, 11184810, 22369621, 44739242, 89478485,
        178956970, 357913941, 715827882, 1431655765,
    ]
    .contains(&n)
}

/// 991. Broken Calculator - `Medium`
///
/// # Idea
/// Backtracking seems to be a better solution than trying to experiment with the starting value
/// that is try to divide by 2 or add 1 to the `target` to get it to `start_value`.
///
/// # Conclusion
/// Backtracking is indeed the correct solution.
///
/// Though my original answer was less optimized with comparisons, with it
/// having the `>` in the if statement, and `!=` as while loop, and as a result no extra `+ (start_value - targ)`,
/// so, due to extra checks, it ended up being significantly slower even though the code difference
/// is not large.
///
/// Still essentially the same solution, and of the course same answer.
pub fn broken_calc(start_value: i32, target: i32) -> i32 {
    let mut targ = target;
    let mut op_num = 0;
    while (targ > start_value) {
        op_num += 1;
        if targ % 2 == 0 {
            targ /= 2
        } else {
            targ += 1
        };
    }
    op_num + (start_value - targ)
}

/// 2016. Maximum Difference Between Increasing Elements - `Easy`
///
/// # Idea
/// Try something similar to 2078.
///
/// # Conclusion
/// _
pub fn maximum_difference(nums: Vec<i32>) -> i32 {
    todo!()
}

/// 2078. Two Furthest Houses With Different Colors - `Easy`
///
/// # Idea
/// Compare initial and final values until they don't match anymore while counting from both sides
/// to cover all cases. Fairly simple.
///
/// # Conclusion
/// This is generally the optimal short solution. So, first guess was correct, getting the right
/// numbers was a bit more annoying.
pub fn max_distance(colors: Vec<i32>) -> i32 {
    let len = colors.len();
    let (mut i, mut l) = (0, len - 1);
    while colors[i] == colors[l] {
        l -= 1;
    }
    while colors[i] == colors[len - 1] {
        i += 1;
    }
    if l > (len - i - 1) {
        l as i32
    } else {
        (len - i - 1) as i32
    }
}

/// 2485. Find the Pivot Integer - `Easy`
///
/// # Idea
/// Brute Force Solution.
///
/// Check sums for each `i` from 1 to `n`, if they are equal that is our solution, otherwise
/// continue until the end where the answer would be -1.
///
/// # Conclusion
/// Better implementations are using the math formula for this, or a similar solution but with
/// sums as updated variables, but this one is easy on the eyes and fairy simple.
///
/// Also seems like a perfect way to showcase zipper comonad.
pub fn pivot_integer(n: i32) -> i32 {
    for i in 1..=n {
        if (1..=i).sum::<i32>() == (i..=n).sum() {
            return i;
        }
    }
    -1
}
