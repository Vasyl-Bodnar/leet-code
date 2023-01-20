// TODO: Try to Improve Comments, beyond best idea and conclusion, with older ideas, more thinking process
//// Solutions are rarely best, I usually keep slower or less efficient but still pretty good personal solution, 
//// instead of just copying a better solution, unless my original was quite bad or did not pass all cases
#![allow(unused)]
use std::cmp::{max, min};
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
    let mut hash: HashMap<i32, i32> = HashMap::new();
    for (i, x) in nums.into_iter().enumerate() {
        if let Some(val) = hash.get(&(target - x)) {
            return vec![i as i32, *val];
        }
        hash.insert(x, i as i32);
    }
    unreachable!()
}

/// 10. Regular Expression Matching - `Hard`
///
/// # Idea
/// _
/// # Conclusion
/// _
pub fn is_match(s: String, p: String) -> bool {
    todo!()
}

/// 153. Find Minimum in Rotated Sorted Array - `Medium`
///
/// # Idea
///
/// ```
/// nums.into_iter().min().unwrap()
/// ```
/// For a serious solution, we just need to implement a `Binary Search`.
/// Since everything is sorted, we just need to add a couple of checks
/// for finding the smallest one.
///
/// # Conclusion
///
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

/// 941. Valid Mountain Array - `Easy`
///
/// # Idea
/// A Simple Solution would be
/// First, array must be more than 3 elements
/// Then, we go through the array and check for constant increase,
/// Once it starts decreasing we switch to checking for constant decrease,
/// If that goes well, then return true, otherwise it goes against the rules
/// and should be false.
///
/// # Conclusion
/// Generally the best solution, though my original answer lacked a couple of optimizations with
/// checks in if statements, but even with them it was not a significant difference.
pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    let len = arr.len();
    if len < 3 || arr[0] >= arr[1] || arr[len - 1] >= arr[len - 2] {
        return false;
    }
    let mut pivot = 0;
    for i in (1..len) {
        if arr[i] > arr[pivot] {
            pivot = i;
        } else {
            break;
        }
    }
    for i in (pivot + 1..len) {
        if arr[i] < arr[pivot] {
            pivot = i
        } else {
            return false;
        }
    }
    true
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

/// 1299. Replace Elements with Greatest Element on Right Side - `Medium`
///
/// # Idea
/// Go through array from right to left, keeping max value, and replacing elements
///
/// # Conclusion
/// Perfect Solution, one optimization is using array instead of a vector as size is known
/// and it can avoid the extra reverse at the end, but implementation itself would still be the same
pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let (mut ans, mut best) = (vec![], -1);
    for i in arr.into_iter().rev() {
        ans.push(best);
        best = max(i, best)
    }
    ans.into_iter().rev().collect()
}

/// 1304. Find N Unique Integers Sum up to Zero - `Easy`
///
/// # Idea
/// Run a generator, which just fills out negative and positive integers on both ends
/// if odd just go from -n/2 to n/2, if even same while ignoring the 0, either through filter or
/// chaining.
///
/// # Conclusion
/// Works nicely, multiple possible implementations here, but I like how simple mine is with ranges
/// in comparison to something like loops or iterator solutions, speed-wise it was still on par
/// with others though.
pub fn sum_zero(n: i32) -> Vec<i32> {
    if n % 2 == 0 {
        (-(n/2)..0).chain(1..=n/2).collect()
    }
    else {
        (-(n/2)..=n/2).collect()
    }
}

/// 2016. Maximum Difference Between Increasing Elements - `Easy`
///
/// # Idea
/// Navigate list from right to left, keep max and min, update min when max is updated to make sure
/// largest to the right is kept. Keep best combo of max and min you know.
///
/// # Conclusion
/// Original idea was correct. This solution specificily works and is nearly equal to the best solution in terms of perfomance and time, however,
/// its shortcomings are being quite more verbose than the best version where we only utilize min, n, and best on a normal unreversed list.
pub fn maximum_difference(nums: Vec<i32>) -> i32 {
    let (i, f) = (nums[0], nums[nums.len() - 1]);
    let (mut min, mut max, mut best) = (f, f - 1, -1);
    for n in nums.into_iter().rev() {
        if n >= max && n != i {
            max = n;
            min = n + 1
        } else if n <= min {
            min = n
        }
        if (max - min) > best {
            best = max - min
        }
    }
    best
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
    (if l > (len - i - 1) { l } else { (len - i - 1) }) as i32
}

/// 2404. Most Frequent Even Element - `Easy`
///
/// # Idea
/// `HashMap`! Certain numbers have to be kept track of and then compared,
/// of course the best solution to this seems to be just using a hashmap.
/// 
/// For this we will try to keep even numbers as keys and how many times they appear as values.
/// # Conclusion
/// The best solution possible, a potential different approach was to use iterators instead of 
/// a single for loop with checks, but, while equal in perfomance, it would not be better in
/// anything except for a couple of saved lines, if even that
pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let (mut best, mut val) = ((-1,-1), 0);
    for num in nums { // could have used .into_iter.filter, and for_each, and etc.
        if num % 2 == 0 {
            val = match map.get(&num) {
                Some(v) => v+1,
                None => 1
            };
            if val > best.1 || (val == best.1 && num < best.0) {
                best = (num, val);
            }
            map.insert(num, val);
        }
    }
    best.0
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
