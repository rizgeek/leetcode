use std::{collections::HashMap, usize};

fn main() {
    println!("Hello, world!");
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let length: usize = nums.len();

    for x in 0..length {
        for y in x+1..length {
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
            return vec![index as i32, i as i32]
        }

        map.insert(*num, i);
    }

    vec![]
}

fn is_palindrome(x: i32) -> bool{
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
       assert_eq!( roman_to_int("III".to_string()), 3);
       assert_eq!( roman_to_int("LVIII".to_string()), 58);
       assert_eq!( roman_to_int("MCMXCIV".to_string()), 1994);
    }


    #[test]
     fn test_is_palindrome() {
       assert_eq!( is_palindrome(-32), false);
       assert_eq!( is_palindrome(32), false);
       assert_eq!( is_palindrome(-32), false);
       assert_eq!( is_palindrome(121), true);
       assert_eq!( is_palindrome(11), true);
       assert_eq!( is_palindrome(22), true);
       assert_eq!( is_palindrome(656), true);
     }

    #[test]
    fn test_two_sum() { 
        assert_eq!(two_sum(vec![2,7,11,15], 9), vec![0,1]);
        assert_eq!(two_sum(vec![3,2,4], 6), vec![1,2]);
        assert_eq!(two_sum(vec![3,3], 6), vec![0,1]);
    }

    #[test]
    fn test_two_sum_optimize() {
        assert_eq!(two_sum_optimize(vec![2,7,11,15], 9), vec![0,1]);
        assert_eq!(two_sum_optimize(vec![3,2,4], 6), vec![1,2]);
        assert_eq!(two_sum_optimize(vec![3,3], 6), vec![0,1]);
    }
}