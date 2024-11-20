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



#[cfg(test)]
mod tests {
    use super::*;

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