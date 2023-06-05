// TODO: Try to Improve Comments, beyond best idea and conclusion, with older ideas, more thinking process
//// Solutions are rarely best, I usually keep slower or less efficient but still pretty good personal solution,
//// instead of just copying a better solution, unless my original was quite bad or did not pass all cases
#![allow(unused)]
use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::Read;
use std::iter;
use std::ops::Neg;
use std::str::Chars;
pub mod util;
use util::*;
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

/// 2. Add Two Numbers - `Medium`
///
/// # Idea
/// Originally I wanted a brute force solution (i.e. convert lists to numbers, add, convert back to list), and I got it to work for almost
/// all cases, except final ones, where unfortunately numbers were too great even for u128.
///
/// This solution is also simple, though not as funny as my original one. Add each node, carry any leftover and that way slowly build your
/// final node.
///
/// # Conclusion
/// This is actually considered a fast 0ms solution, and I believe it should be O(max(v,u)), where v is
/// len of first list, u is len of second list, just like other good solutions.
///
/// A better solution in terms of visual looks, though not much different in terms of speed,
/// would be a single while loop variation which matches until both lists are null and
/// there is no carry.
///
/// There are mirror solutions in rust, however,
/// I see no point in just rewriting it that way, so I will just keep this one.
///
/// # Note
/// Rust collections has a LinkedList definition, but it is double linked which
/// is not the type of list the problem offers, not that it would make much of a difference.
pub fn add_two_numbers(
    mut l1: Option<Box<ListNode<i32>>>,
    mut l2: Option<Box<ListNode<i32>>>,
) -> Option<Box<ListNode<i32>>> {
    impl ListNode<i32> {
        pub fn append(&mut self, node: &Option<Box<ListNode<i32>>>) {
            if let Some(ref mut n) = self.next {
                n.append(node)
            } else {
                self.next = node.clone()
            }
        }
    }
    fn get_val(node: &Option<Box<ListNode<i32>>>) -> Option<i32> {
        if let Some(n) = node {
            Some(n.val)
        } else {
            None
        }
    }
    let mut lf = ListNode::new(0);
    let mut next = Some(Box::new(ListNode::new(0)));
    let mut add_res = (0, 0);
    let mut add = |ad: i32| {
        let addition = ad + add_res.1;
        add_res = if addition < 10 {
            (addition, 0)
        } else {
            (addition % 10, 1)
        };
        next.as_mut().unwrap().val += add_res.0;
        lf.append(&next);
        next = Some(Box::new(ListNode::new(0)));
    };
    loop {
        match (get_val(&l1), get_val(&l2)) {
            (Some(v), Some(u)) => {
                l1 = l1.unwrap().next;
                l2 = l2.unwrap().next;
                add(v + u)
            }
            (Some(v), None) => {
                l1 = l1.unwrap().next;
                add(v)
            }
            (None, Some(u)) => {
                l2 = l2.unwrap().next;
                add(u)
            }
            _ => break,
        }
    }
    if add_res.1 != 0 {
        next.as_mut().unwrap().val = add_res.1;
        lf.append(&next);
    }
    lf.next
}

/// 3. Longest Substring Without Repeating Characters - `Medium`
///
/// # Idea
/// Sliding Window, keeping the last non-repeating character as start, and iterating over the the
/// string.
///
/// # Conclusion
/// _
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut pos = [0; 128];
    let (mut fin, mut i) = (0, 0);
    for (j, ch) in s.as_bytes().into_iter().enumerate() {
        i = max(i, pos[*ch as usize]);
        fin = max(fin, j - i + 1);
        pos[*ch as usize] = j + 1;
    }
    return fin as i32;
}

/// 4. Median of Two Sorted Arrays - `Hard`
///
/// # Idea
/// Using the fact that given arrays are very nice and extendable,
/// we do not need to sort the mix, but just insert elements with comparisons
/// for automatic sorting. Overall a very simple solution of `create array` and `take median`.
///
/// # Conclusion
/// This is a very fast solution, original was literal chaining and direct sorting, and same median calculation,
/// which was slower, but not by much. Which was only because I did not notice that arrays were already sorted.
/// Again, very simple problem, and very simple solution, so hard to believe this is a `Hard`
/// problem.
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut mix = vec![];
    let (mut i, mut j) = (0, 0);
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] < nums2[j] {
            mix.push(nums1[i]);
            i += 1;
        } else {
            mix.push(nums2[j]);
            j += 1;
        }
    }
    while i < nums1.len() {
        mix.push(nums1[i]);
        i += 1;
    }
    while j < nums2.len() {
        mix.push(nums2[j]);
        j += 1;
    }
    let len = mix.len();
    if len % 2 == 0 {
        (mix[len / 2 - 1] + mix[len / 2]) as f64 / 2f64
    } else {
        mix[len / 2] as f64
    }
}

/// 8. String to Integer (atoi) - `Medium`
///
/// # Idea
/// Tried iterators, and as always in these cases they are a bit too restrictive and lack certain
/// methods to complete this with ease.
///
/// # Conclusion
/// _
pub fn my_atoi(s: String) -> i32 {
    let (mut start, mut fin, mut pos) = (false, 0i64, true);
    for c in s.bytes() {
        match c {
            b'0'..=b'9' => {
                start = true;
                fin = fin * 10 + (c - 48) as i64;
                if fin > i32::MAX as i64 {
                    break;
                }
            }
            _ if start => break,
            b' ' => continue,
            b'+' => {
                pos = true;
                start = true;
            }
            b'-' => {
                pos = false;
                start = true;
                println!("{pos}")
            }
            _ => break,
        }
    }
    if pos { fin } else { -fin }.clamp(i32::MIN.into(), i32::MAX.into()) as i32
}

/// 10. Regular Expression Matching - `Hard`
///
/// # Idea
/// Simple dynamic programming.
///
/// # Conclusion
/// Very fast and natural solution, though I am still learning how to do this properly.
pub fn is_match(s: String, p: String) -> bool {
    let (s, p) = (s.as_bytes(), p.as_bytes());
    let (sl, pl) = (s.len(), p.len());
    let mut dp = vec![vec![false; pl + 1]; sl + 1];
    dp[0][0] = true;
    for i in 1..=pl {
        if p[i - 1] == b'*' {
            dp[0][i] = dp[0][i - 2];
        }
    }
    for i in 1..=sl {
        for j in 1..=pl {
            if p[j - 1] == b'.' || p[j - 1] == s[i - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else if p[j - 1] == b'*' {
                dp[i][j] = dp[i][j - 2];
                if p[j - 2] == b'.' || p[j - 2] == s[i - 1] {
                    dp[i][j] |= dp[i - 1][j];
                }
            }
        }
    }
    dp[sl][pl]
}

/// 12. Integer to Roman - `Medium`
///
/// # Idea
/// Zip possible letters with their numerical values, find first one that matches and
/// returns the modulo with itself, build it and continue recursively.
///
/// # Conclusion
/// Speed-wise it was somewhat random, certain times it was on a slower end at 7-9ms othertimes it
/// was fastest at 0-2ms. Somewhat better solutions were not much less hacky than mine in idea, built arrays
/// of all possible values and divided and modulo number into indexes to get their answer.
///
/// Rust is not best for recursion, and this could be done pretty well without it, but I decided that it is fine
/// and I would forget if I did not use for long time.
pub fn int_to_roman(num: i32) -> String {
    fn f(num: i32) -> String {
        if num == 0 {
            return String::from("");
        }
        if let Some((l, n)) = "MDCLXVI"
            .chars()
            .zip([1000, 500, 100, 50, 10, 5, 1])
            .find(|(_, n)| n % num == *n || num == *n)
        {
            (String::from(l) + &*f(num - n))
        } else {
            String::from("")
        }
    };
    f(num)
        .replace("IIII", "IV")
        .replace("XXXX", "XL")
        .replace("CCCC", "CD")
        .replace("VIV", "IX")
        .replace("LXL", "XC")
        .replace("DCD", "CM")
}

/// 13. Roman to Integer - `Easy`
///
/// # Idea
/// Convert string to list of ints, fold based on
/// "if previous number is smaller than next one then substract, else add".
///
/// # Conclusion
/// Very simple a nice solution, it is slower than other solutions, which rely on heavily
/// engineered loops and hashmaps, however I would still prefer to write this one, unless
/// perfomance was critical.
///
/// # Extra
/// Another solution I tried was to replace all possible unique instances with their direct
/// counterparts (i.e. IV -> IIII) and then just map them with numbers and sum.
/// It turns out that is the fastest solution, despite how simple it is.
/// I still choose to keep my older solution, since it looks nicer and less hacky.
pub fn roman_to_int(s: String) -> i32 {
    s.chars()
        .map(|ch| match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        })
        .rfold((0, 0), |(acc, prev), num| {
            if prev > num {
                (acc - num, num)
            } else {
                (acc + num, num)
            }
        })
        .0
}

/// 14. Longest Common Prefix - `Easy`
///
/// # Idea
/// Run over chars in the first string and compare it with other chars at same location in other strings,
/// if matched, add to our prefix string, otherwise just return it immediately.
///
/// # Conclusion
/// Overall, it is a good solution, has good time, and is basically the same for all top answers.
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut pref = String::from("");
    for (i, ch) in strs[0].char_indices() {
        for elem in strs.iter().map(|x| x.chars().nth(i)) {
            if elem != Some(ch) {
                return pref;
            }
        }
        pref.push(ch)
    }
    pref
}

/// 32. Longest Valid Parentheses - `Hard`
///
/// # Idea
/// Go through the string, and check for all parentheses pairs, zeroing in case of broken pairs (i.e.
/// "((" or "))"), then do the same thing for reverse string.
/// # Conclusion
/// Simple and fast solution, which unlike a stack solution, uses basically no space at all.
pub fn longest_valid_parentheses(s: String) -> i32 {
    let (mut left, mut right, mut fin) = (0, 0, 0);
    let mut loops = |mut left: i32,
                     mut right: i32,
                     chs: Box<dyn Iterator<Item = char>>,
                     comp: fn(i32, i32) -> bool| {
        for x in chs {
            match x {
                '(' => left += 1,
                _ => right += 1,
            }
            if left == right {
                fin = max(left * 2, fin);
            } else if comp(left, right) {
                (left, right) = (0, 0);
            }
        }
    };
    loops(left, right, Box::new(s.chars()), |l, r| r > l);
    (left, right) = (0, 0);
    loops(left, right, Box::new(s.chars().rev()), |l, r| l > r);
    fin
}

/// 38. Count and Say - `Medium`
///
/// # Idea
/// Rather simple solution, if 1 then return 1, else recurse, for each recursion,
/// handle the output by counting and appending count and ch when ch changes, or loop ends.
///
/// # Conclusion
/// Though I believed it was a simple but nice solution, this was actually the fastest 0ms solution, better than
/// 95% in this problem, and generally this is how this problem is solved for all cases.
pub fn count_and_say(n: i32) -> String {
    if n == 1 {
        return String::from("1");
    }
    let mut fin = String::new();
    let word = count_and_say(n - 1);
    let bytes = word.as_bytes();
    let (mut cur, mut count) = (bytes[0], 1);
    for ch in bytes[1..].into_iter() {
        if cur == *ch {
            count += 1;
        } else {
            fin += &*format!("{count}{}", cur as char);
            cur = *ch;
            count = 1;
        }
    }
    format!("{fin}{count}{}", cur as char)
}

/// 54. Spiral Matrix - `Medium`
///
/// # Idea
/// Take out the edges, and recurse inside, handle the basic cases, where the solution is just
/// flattening it.
///
/// # Conclusion
/// Generally solid solution and is relatively fast
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.len() < 2 || matrix[0].len() < 2 {
        return matrix.into_iter().flatten().collect();
    }
    let mut fin = matrix[0].clone();
    let last = matrix.len() - 1;
    for i in 1..last {
        if let Some(v) = matrix[i].last() {
            fin.push(*v)
        }
    }
    fin.extend(&mut matrix[last].iter().rev());
    for i in (1..last).rev() {
        if let Some(v) = matrix[i].first() {
            fin.push(*v)
        }
    }
    fin.append(&mut spiral_order(
        matrix[1..last]
            .into_iter()
            .map(|arr| arr[1..arr.len() - 1].to_owned())
            .collect(),
    ));
    fin
}

/// 64. Minimum Path Sum - `Medium`
///
/// # Idea
/// Dynamic Programming Examplum
///
/// # Conclusion
/// As expected, this is a good solution, I used a more basic version before this, since I forgot I
/// can just use original array without creating a new one
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }
    let m = grid.len();
    let n = grid[0].len();
    for i in 0..m {
        for j in 0..n {
            match (i, j) {
                (0, 0) => {}
                (0, j) => grid[0][j] += grid[0][j - 1],
                (i, 0) => grid[i][0] += grid[i - 1][0],
                (i, j) => grid[i][j] += min(grid[i - 1][j], grid[i][j - 1]),
            }
        }
    }
    grid[m - 1][n - 1]
}

/// 65. Valid Number - `Hard`
///
/// # Idea
/// A poor man's parser combinators
///
/// # Conclusion
/// Generally there were a lot of ways to do this, as despite being `Hard`, it is a fairly easy
/// problem, a fast solution would be to use a bunch of flags and a loop, a smarter (and slower), but insaner
/// solution would be more like this. You could also acomplish this abusing Options directly instead of
/// fancy and verbose function combinations, but I wanted to do it this way, and I will likely still try to improve
/// this in a future.
pub fn is_number(s: String) -> bool {
    type Parsed = Option<(Vec<u8>, Vec<u8>)>;
    // Hack for trait aliasing
    trait Parser: FnMut(Vec<u8>) -> Parsed + Clone {}
    impl<T: FnMut(Vec<u8>) -> Parsed + Clone> Parser for T {}
    fn eof(mut f: impl Parser, inp: Vec<u8>) -> bool {
        match f(inp) {
            Some((_, vs)) if vs.is_empty() => true,
            _ => false,
        }
    }
    fn satisfy(mut b: impl FnMut(u8) -> bool + Clone) -> impl Parser {
        move |inp: Vec<u8>| {
            let mut s = inp.into_iter();
            match s.next() {
                Some(v) if b(v) => Some((vec![v], s.collect())),
                _ => None,
            }
        }
    }
    fn and(mut f: impl Parser, mut g: impl Parser) -> impl Parser {
        move |inp| {
            if let Some((mut v, vs)) = f(inp) {
                if let Some((u, us)) = g(vs) {
                    v.extend(u);
                    return Some((v, us));
                }
            }
            None
        }
    }
    fn or(mut f: impl Parser, mut g: impl Parser) -> impl Parser {
        move |inp: Vec<u8>| {
            if let Some((mut v, vs)) = f(inp.clone()) {
                Some((v, vs))
            } else {
                g(inp)
            }
        }
    }
    fn both_or_f(mut f: impl Parser, mut g: impl Parser) -> impl Parser {
        move |inp: Vec<u8>| {
            if let Some((mut v, vs)) = f(inp.clone()) {
                if let Some((u, us)) = g(vs) {
                    v.extend(u);
                    return Some((v, us));
                }
            }
            f(inp)
        }
    }
    fn both_or_g(mut f: impl Parser, mut g: impl Parser) -> impl Parser {
        move |inp: Vec<u8>| {
            if let Some((mut v, vs)) = f(inp.clone()) {
                if let Some((u, us)) = g(vs) {
                    v.extend(u);
                    return Some((v, us));
                }
            }
            g(inp)
        }
    }
    fn many(mut f: impl Parser) -> impl Parser {
        move |mut inp: Vec<u8>| {
            let mut fin = Vec::new();
            loop {
                if let Some(outp) = f(inp.clone()) {
                    fin.extend(outp.0);
                    inp = outp.1;
                } else {
                    return Some((fin, inp));
                }
            }
        }
    }
    fn some(mut b: impl Parser) -> impl Parser {
        and(b.clone(), many(b))
    }
    fn optional(mut f: impl Parser, mut g: impl Parser) -> impl Parser {
        both_or_g(f, g)
    }
    fn sign(ch: Vec<u8>) -> impl Parser {
        satisfy(move |x| ch.contains(&x))
    }
    fn integer() -> impl Parser {
        optional(
            sign(vec![b'+', b'-']),
            some(satisfy(|x| x.is_ascii_digit())),
        )
    }
    fn decimal() -> impl Parser {
        let int = some(satisfy(|x| x.is_ascii_digit()));
        optional(
            sign(vec![b'+', b'-']),
            or(
                optional(int.clone(), and(sign(vec![b'.']), int.clone())),
                and(int, sign(vec![b'.'])),
            ),
        )
    }
    eof(
        both_or_f(
            or(decimal(), integer()),
            and(sign(vec![b'e', b'E']), integer()),
        ),
        s.bytes().collect(),
    )
}

/// 67. Add Binary - `Easy`
///
/// # Idea
/// Using iterations, and carry variable, we can simulate natural binary addition,
/// First we append zeroes to smaller string, then we add each element, keeping wary of
/// carry, and at the end, we pad ones from carry and turn it into a String from bytes
///
/// # Conclusion
/// Simple solution with fast speed, and pure iteration, excluding some impure carry.
/// This could probably be imporoved size wise, but the only real better solution, was
/// simply using rust's method for converting binary string to number, addition, and number to string,
/// which is pure cheating. Still same speed wise somehow.
pub fn add_binary(a: String, b: String) -> String {
    let mut carry = 0u8;
    if a.len() > b.len() {
        a.as_bytes()
            .into_iter()
            .rev()
            .zip(b.as_bytes().iter().rev().chain(iter::repeat(&b'0')))
    } else {
        b.as_bytes()
            .into_iter()
            .rev()
            .zip(a.as_bytes().iter().rev().chain(iter::repeat(&b'0')))
    }
    .fold(String::new(), |mut acc, chs| {
        match chs {
            (b'0', b'0') => acc.push(if carry >= 1 {
                carry -= 1;
                '1'
            } else {
                '0'
            }),
            (b'1', b'0') | (b'0', b'1') => acc.push(if carry >= 1 { '0' } else { '1' }),
            (b'1', b'1') => acc.push(if carry >= 1 {
                '1'
            } else {
                carry += 1;
                '0'
            }),
            _ => unreachable!(),
        };
        acc
    })
    .as_bytes()
    .into_iter()
    .chain(iter::repeat(&b'1').take(carry as usize))
    .collect::<Vec<&u8>>()
    .into_iter()
    .rev()
    .map(|x| if *x == b'1' { '1' } else { '0' })
    .collect()
}

/// 71. Simplify Path - `Medium`
///
/// # Idea
/// Very simple idea, go in reverse through the path, keep track of how many times back we have to
/// go (`..`), and if it is above one, then the next folder (prev from left to right) would simply be
/// removed. `.` and `/` can simply be ignored and `/` will be automatically added to each folder name.
/// At the end there is reverse, concat and final check, so in case there were no folders it would just
/// give root.
///
/// # Conclusion
/// Overall one of the fastest solutions, most people had something similar in nature, even if
/// methods and certain specifics were different.
pub fn simplify_path(path: String) -> String {
    let mut fin = vec![];
    let mut back = 0;
    for s in path.split('/').rev() {
        match s {
            ".." => {
                back += 1;
            }
            "." | "" => (),
            _ if back > 0 => {
                back -= 1;
            }
            _ => {
                fin.push(s);
                fin.push("/")
            }
        };
    }
    fin.reverse();
    let fin = fin.concat();
    if !fin.contains('/') {
        String::from("/")
    } else {
        fin
    }
}

/// 72. Edit Distance - `Hard`
///
/// # Idea
/// Work in Progress
///
/// # Conclusion
/// _
pub fn min_distance(word1: String, word2: String) -> i32 {
    let (dist, diff) = (
        (word1.len() - word2.len()),
        word1
            .as_bytes()
            .into_iter()
            .zip(word2.as_bytes())
            .map(|(a, b)| a == b)
            .fold(0, |acc, x| if x { acc + 1 } else { acc }),
    );
    dist as i32 + diff as i32
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

/// 168. Excel Sheet Column Title - `Easy`
///
/// # Idea
/// General idea is that we simply divide the column number by 26 and test what letter we are at
/// with modulo operator, then we append to our result string.
///
/// Since I just append to the string, we have to reverse this at the end.
///
/// # Conclusion
/// Fine fast solution, one improvement would be to not append but instead generate an array of base 26
/// numbers and then convert them to letters in a map.
pub fn convert_to_title(column_number: i32) -> String {
    let (mut fin, mut cl) = (String::from(""), column_number);
    while cl > 0 {
        cl -= 1;
        fin += &*(((cl % 26 + 'A' as i32) as u8 as char).to_string());
        cl /= 26;
    }
    fin.chars().rev().collect()
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
/// in case if it is considered cheating, then the generator or the idea from it works well. As it
/// is just meant as a simple bit manipulation in the end.
pub fn has_alternating_bits(n: i32) -> bool {
    [
        2, 5, 10, 21, 42, 85, 170, 341, 682, 1365, 2730, 5461, 10922, 21845, 43690, 87381, 174762,
        349525, 699050, 1398101, 2796202, 5592405, 11184810, 22369621, 44739242, 89478485,
        178956970, 357913941, 715827882, 1431655765,
    ]
    .contains(&n)
}

/// 877. Stone Game - `Medium`
///
/// # Idea
/// ```
/// true
/// ```
/// Otherwise, assuming that not every answer is true
/// Check two possibilities, predict higher for Alice and Bob, calculate gains
/// simple loop, beautfully simple
///
/// # Conclusion
/// Generally the two solutions are dynamic programming and the recursion,
/// I personally done recursion, though naturally converted to an iterative approach for better
/// perfomance and because some of the inputs require a lot of depth which is just not going to
/// play nice with Rust.
pub fn stone_game(piles: Vec<i32>) -> bool {
    let (mut piles, mut alice, mut gains) = (piles, true, 0);
    loop {
        if piles.is_empty() {
            break;
        }
        if alice {
            gains += max(
                if let Some((x, xs)) = piles.split_first() {
                    let x = *x;
                    piles = xs.to_vec();
                    alice = !alice;
                    x
                } else {
                    0
                },
                if let Some((x, xs)) = piles.split_last() {
                    let x = *x;
                    piles = xs.to_vec();
                    alice = !alice;
                    x
                } else {
                    0
                },
            )
        } else {
            gains -= max(
                if let Some((x, xs)) = piles.split_first() {
                    let x = *x;
                    piles = xs.to_vec();
                    alice = !alice;
                    x
                } else {
                    0
                },
                if let Some((x, xs)) = piles.split_last() {
                    let x = *x;
                    piles = xs.to_vec();
                    alice = !alice;
                    x
                } else {
                    0
                },
            )
        }
    }
    if gains > 0 {
        true
    } else {
        false
    }
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
/// Though my original answer was less optimized with comparisons, so with some extra checks,
/// it ended up being significantly slower even though the code difference is not large.
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

/// 1011. Capacity To Ship Packages Within D Days - `Medium`
///
/// # Idea
/// `Binary Search`, nothing more, nothing less
///
/// # Conclusion
/// Decently fast, faster solutions were unrolling some things and
/// complicating the process, which is not really important for the task.
pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    let mut left: i32 = *weights.iter().max().unwrap();
    let mut right: i32 = weights.iter().sum();
    while left < right {
        let (mid, mut count, mut sm) = ((left + right) / 2, 1, 0);
        for weight in weights.iter() {
            sm += weight;
            if sm > mid {
                count += 1;
                sm = *weight;
            }
        }
        if count > days {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

/// 1140. Stone Game II - `Medium`
///
/// # Idea
/// _
///
/// # Conclusion
/// _
pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    let mut piles = piles;
    let mut m = 1;
    let mut alice = true;
    let (mut gain, mut alice_gains, mut bob_gains) = (vec![], 0, 0);
    loop {
        if piles.is_empty() {
            break;
        }
        if m > piles.len() {
            if alice {
                return alice_gains + piles.into_iter().sum::<i32>();
            } else {
                break;
            }
        }
        for x in 1..=2 * m {
            let res: i32 = piles.iter().take(x).sum();
            gain.push((x, res));
        }
        let best = gain
            .iter()
            .reduce(|acc, res| if res.1 > acc.1 { res } else { acc })
            .unwrap();
        if alice {
            alice_gains += best.1;
        } else {
            bob_gains += best.1;
        }
        alice ^= true;
        piles = piles[best.0..].to_vec();
        m = max(m, best.0);
        //dbg!(&piles);
    }
    alice_gains
}

/// 1232. Check If It Is a Straight Line - `Easy`
///
/// # Idea
/// Original Attempt:
/// Make a line equation and use it to check each point by whether they satisfy it or not, have to
/// also explicitly handle the case where x does not change, that is a vertical line
///
/// New Attempt:
/// Take 3 points, and check if their slopes match as expected
/// # Conclusion
/// In general both solution had essentially the same speed and memory, though the newer one is
/// smaller and a bit more clear, especially if function is extracted and matched into nicer xs and
/// ys
pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    // match (&coordinates[0][..], &coordinates[1][..]) {
    //     ([x1, y1], [x2, y2]) if x2 == x1 => {
    //         let i = *x1;
    //         coordinates.into_iter().all(|p| p[0] == i)
    //     }
    //     ([x1, y1], [x2, y2]) => {
    //         let m = ((y2 - y1) as f32) / ((x2 - x1) as f32);
    //         let i = (*y1 as f32) - m * (*x1 as f32);
    //         coordinates
    //             .into_iter()
    //             .all(|p| p[1] as f32 == m * (p[0] as f32) + i)
    //     }
    //     _ => unreachable!(),
    // }
    coordinates.windows(3).all(|ps| {
        (ps[1][0] - ps[0][0]) * (ps[2][1] - ps[1][1])
            == (ps[2][0] - ps[1][0]) * (ps[1][1] - ps[0][1])
    })
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
        (-(n / 2)..0).chain(1..=n / 2).collect()
    } else {
        (-(n / 2)..=n / 2).collect()
    }
}

/// 1455. Check If a Word Occurs As a Prefix of Any Word in a Sentence - `Easy`
///
/// # Idea
/// Split words by space, loop until you find a prefix, otherwise return -1.
/// Very easy and simply solution.
///
/// # Conclusion
/// Originally implemented with a simple for loop over split and enumerated words, and then
/// with an pure iterator for comparison. Solutions are identical in how they work,
/// though interestingly iterator solutions is slightly faster, either way this is a
/// simply and efficient solution to the problem.
pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    // for (i,w) in sentence.split_whitespace().enumerate() {
    //     if w.starts_with(&search_word) {
    //          return (i+1) as i32
    //     }
    // }
    // -1
    sentence
        .split_whitespace()
        .enumerate()
        .find_map(|(i, w)| {
            if w.starts_with(&search_word) {
                Some((i + 1) as i32)
            } else {
                None
            }
        })
        .unwrap_or(-1)
}

/// 1636. Sort Array by Increasing Frequency - `Easy`
///
/// # Idea
/// `HashMap` as it is one of the best for counting frequencies.
/// As for sorting by frequencies, I will use a bit weird solution,
/// with an array where I shall insert all of my hashmap values,
/// and sort them to get the right placements.
///
/// # Conclusion
/// Overall, it is a bit of an insane solution, since it will probably waste space and does not scale with array.
/// Generally, at least in Rust, the solution is to fill the hashmap, and then use a sort method
/// on original array which is based on the frequency and value.
/// However, I can always replace array with another hashmap, and for now, mine is faster, with only faster submission being excessive in size.
pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        map.insert(num, map.get(&num).unwrap_or(&0) + 1);
    }
    const EMPTY: Vec<i32> = Vec::new();
    let mut arr = [EMPTY; 100];
    for (k, v) in map {
        arr[(v - 1) as usize].push(k);
    }
    arr.into_iter()
        .enumerate()
        .flat_map(|(i, mut x)| {
            x.sort_by_key(|x| -(x));
            x.into_iter()
                .flat_map(move |y| std::iter::repeat(y).take(i + 1))
        })
        .collect()
}

/// 1678. Goal Parser Interpretation - `Easy`
///
/// # Idea
/// ```
/// command.replace("()", "o").replace("(al)", "al")
/// ```
/// Jokes aside, there are two ways to approach this problem, first is to
/// just go over the array and setup a flag for open paren, which you can then
/// match for o or all depending on next element. The other, arguably better one,
/// at least in speed, is windows, which I like and wanted to use for a while.
///
/// # Conclusion
/// It is indeed one of the best solutions in speed and beauty,
/// though `command.replace.replace` is actual best as it is most clear and light.
pub fn interpret(command: String) -> String {
    command
        .as_bytes()
        .windows(2)
        .map(|chs| match chs {
            [b'G', _] => "G",
            [b'(', b')'] => "o",
            [b'(', b'a'] => "al",
            _ => "",
        })
        .collect::<String>()
        + if command.ends_with("G") { "G" } else { "" }
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

/// 2055. Plates Between Candles - `Medium`
///
/// # Idea
/// First we find all indices of candles, then we map over the query,
/// using binary search to get candles on the right and left,
/// our answer would be either all values between those two candles -
/// the count of candles between, or just 0 as not enough candles are found.
///
/// # Conclusion
/// It is a bit trickier solution requiring either an efficient binary search or
/// a bit better solution based on three collecting vectors.
///
/// My original attempt was done purely with iterators without binary search,
/// however it is unable to pass some of the more massive examples due to poorer scaling.
pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let pos: Vec<_> = s
        .chars()
        .enumerate()
        .filter_map(|(idx, ch)| if ch == '|' { Some(idx) } else { None })
        .collect();

    queries
        .into_iter()
        .map(|q| {
            let left = pos.binary_search(&(q[0] as usize)).unwrap_or_else(|e| e);
            let right = pos
                .binary_search(&(q[1] as usize))
                .map_or_else(|e| if e == 0 { e } else { e - 1 }, |v| v);
            if left < right {
                (pos[right] - pos[left] - (right - left)) as i32
            } else {
                0
            }
        })
        .collect()
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

/// 2206. Divide Array Into Equal Pairs - `Easy`
///
/// # Idea
/// `HashMap` strikes again, to satisfy both conditions you need to keep count of all numbers,
/// which is a perfect job for a `HashMap`.
///
/// # Conclusion
/// Good solution, some other potential examples were sorting and navigating chunks of 2, which
/// was interesting, considering that using sort seems a bit counterintuitive.
/// Still, this solution is generally the perfect answer, and just like most is O(n).
pub fn divide_array(nums: Vec<i32>) -> bool {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        map.insert(num, map.get(&num).unwrap_or(&0) + 1);
    }
    map.values().all(|x| x % 2 == 0)
}

/// 2390. Removing Stars From a String - `Medium`
///
/// # Idea
/// If we loop from right, we can just have a single integer that keeps track of how many letters
/// to remove. This would be done by simply having stars add to integer, and letters requiring
/// removal to substract from integer.
///
/// # Conclusion
/// Essentially a perfect solution, speed-wise it is one of the fastest, and all the other
/// solutions still use the same model anyway.
pub fn remove_stars(s: String) -> String {
    let mut fin = Vec::new();
    let mut stars = 0;
    for c in s.bytes().rev() {
        match c {
            b'*' => stars += 1,
            _ if stars > 0 => stars -= 1,
            _ => fin.push(c),
        }
    }
    fin.into_iter().rev().map(|c| c as char).collect()
}

/// 2404. Most Frequent Even Element - `Easy`
///
/// # Idea
/// `HashMap`! Certain numbers have to be kept track of and then compared,
/// of course the best solution to this seems to be just using a `Hashmap`.
///
/// For this we will try to keep even numbers as keys and how many times they appear as values.
///
/// # Conclusion
/// The best solution possible, a potential different approach was to use iterators instead of
/// a single for loop with checks, but, while similar in perfomance, it does not seem necessary
/// here.
pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let (mut best, mut val) = ((-1, -1), 0);
    for num in nums {
        // could have used .into_iter.filter, and for_each, and etc.
        if num % 2 == 0 {
            val = match map.get(&num) {
                Some(v) => v + 1,
                None => 1,
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
