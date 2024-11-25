use std::collections::HashSet;
use std::{collections::HashMap, usize};

fn main() {
    println!("Hello, world!");
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let length: usize = nums.len();

    for x in 0..length {
        for y in x + 1..length {
            let check = nums[x] + nums[y];
            if check == target {
                return vec![x as i32, y as i32];
            }
        }
    }
    vec![]
}

fn two_sum_optimize(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let complement: i32 = target - num;

        if let Some(&index) = map.get(&complement) {
            return vec![index as i32, i as i32];
        }

        map.insert(*num, i);
    }

    vec![]
}

fn is_palindrome(x: i32) -> bool {
    let convert: String = x.to_string();
    let revers: &String = &convert.chars().rev().collect();

    if convert == *revers {
        return true;
    }

    false
}

fn roman_to_int(s: String) -> i32 {
    s.chars().rfold(0, |acc, c| {
        acc + match c {
            'I' if acc >= 5 => -1,
            'I' => 1,
            'V' => 5,
            'X' if acc >= 50 => -10,
            'X' => 10,
            'L' => 50,
            'C' if acc >= 500 => -100,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    })
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    let len_vec = strs.len();

    let mut strings = strs;
    strings.sort_by_key(|s| -(s.len() as isize));

    let last = strings.last().unwrap();

    if last.len() <= 0 {
        return "".to_string();
    }

    let expected = (&last[0..(last.len())]).to_string();

    for idx in (0..expected.len()).rev() {
        let prefix = expected[0..&idx + 1].to_string();
        let filter: Vec<_> = strings
            .iter()
            .filter(|&word| word.starts_with(&prefix))
            .collect();

        if len_vec == filter.len() {
            return prefix;
        }
    }

    "".to_string()
}

fn is_valid_parentheses(s: String) -> bool {
    if (s.len() % 2) == 1 {
        return false;
    };

    let parentheses_map: HashMap<char, char> = HashMap::from([(')', '('), ('}', '{'), (']', '[')]);

    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        if parentheses_map.values().any(|&v| v == ch) {
            stack.push(ch);
        } else if let Some(&open) = parentheses_map.get(&ch) {
            if stack.pop() != Some(open) {
                return false;
            }
        }
    }

    stack.is_empty()
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut merged: Vec<i32> = Vec::new();
    merged.extend_from_slice(&nums1);
    merged.extend_from_slice(&nums2);
    merged.sort();

    let len_merged = merged.len();
    let mid = (len_merged as f64 / 2.0).round() as usize;

    if len_merged % 2 == 0 {
        return (merged[mid] + merged[mid - 1]) as f64 / 2.0;
    } else {
        return merged[mid - 1] as f64;
    }
}

fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let len_digit = digits.len();
    let mut temp = digits;

    for i in (0..len_digit).rev() {
        if temp[i] != 9 {
            temp[i] += 1;
            return temp;
        }
        temp[i] = 0;
    }
    temp.insert(0, 1);
    temp
}

fn reverse(x: i32) -> i32 {
    let mut reversed: String = x.to_string();
    reversed = reversed.chars().rev().collect();

    if x < 0 {
        reversed.pop();
        reversed.insert(0, '-');
    }

    if let Some(first_char) = reversed.chars().next() {
        if first_char == '0' {
            reversed.remove(0);
        }
    }

    let result = reversed.parse::<i32>().unwrap_or_else(|_| 0);

    result
}

fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut unique: Vec<_> = nums;
    unique.retain(|&x| x > 0);
    unique = unique
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    unique.sort();

    if unique.len() < 1 {
        return 1;
    }

    let mut result: i32 = 1;

    for &num in &unique {
        if num == result {
            result += 1;
        } else if num > result {
            break;
        }
    }

    result
}

fn first_missing_positive_optimize(mut nums: Vec<i32>) -> i32  {
    let mut last = nums.len() - 1;
    let mut cur: usize = 0;
    while cur < last {
        if nums[cur] <= 0 || nums[cur] as usize > last + 1 {
            nums.swap(cur, last);
            last -= 1;
        } else {
            let val = nums[cur] as usize - 1;
            if val == cur {
                cur += 1;
            } else if val as i32 == nums[val] - 1 {
                nums.swap(cur, last);
                last -= 1;
            } else {
                nums.swap(cur, val);
            }
        }
    }

    for (i, v) in nums.iter().enumerate() {
        if i + 1 != *v as usize {
            return i as i32 + 1;
        }
    }
    nums.len() as i32 + 1
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_missing_positive() {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(first_missing_positive(vec![1, 1, 2, 0]), 3);
        assert_eq!(first_missing_positive(vec![-1]), 1);
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
        assert_eq!(first_missing_positive(vec![1]), 2);
        assert_eq!(first_missing_positive(vec![1, 2, 3, 4, 5, 7, 900]), 6);
        assert_eq!(first_missing_positive(vec![1, 2, 3, 4, 5, 7, 900]), 6);
        
    }

    #[test]
    fn test_first_missing_positive_optimize() {
        assert_eq!(first_missing_positive_optimize(vec![1, 2, 0]), 3);
        assert_eq!(first_missing_positive_optimize(vec![1, 1, 2, 0]), 3);
        assert_eq!(first_missing_positive_optimize(vec![-1]), 1);
        assert_eq!(first_missing_positive_optimize(vec![3, 4, -1, 1]), 2);
        assert_eq!(first_missing_positive_optimize(vec![7, 8, 9, 11, 12]), 1);
        assert_eq!(first_missing_positive_optimize(vec![1]), 2);
        assert_eq!(first_missing_positive_optimize(vec![1, 2, 3, 4, 5, 7, 900]), 6);
        assert_eq!(first_missing_positive_optimize(vec![1, 2, 3, 4, 5, 7, 900]), 6);
        
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(321), 123);
        assert_eq!(reverse(1230), 321);
        assert_eq!(reverse(-1230), -321);
        assert_eq!(reverse(-99993232), -23239999);
    }

    #[test]
    fn test_plus_one() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
        assert_eq!(
            plus_one(vec![
                9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9
            ]),
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_find_median_sorted_arrays() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let actual = find_median_sorted_arrays(nums1, nums2);
        let expected = 2.00000;

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_is_valid_parentheses() {
        let str: String = "{(())[]}".to_string();
        is_valid_parentheses(str);
    }

    #[test]
    fn test_longest_common_prefix() {
        let strs: Vec<String> = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
            "flight".to_string(),
        ];

        assert_eq!(longest_common_prefix(strs), "fl".to_string());
    }

    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(-32), false);
        assert_eq!(is_palindrome(32), false);
        assert_eq!(is_palindrome(-32), false);
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(11), true);
        assert_eq!(is_palindrome(22), true);
        assert_eq!(is_palindrome(656), true);
    }

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_optimize() {
        assert_eq!(two_sum_optimize(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum_optimize(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum_optimize(vec![3, 3], 6), vec![0, 1]);
    }
}
