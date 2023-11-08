use crate::solution::{ Solution, Runner };
use std::collections::HashMap;

pub struct TwoSum {
  solution: Solution,
}

impl TwoSum {
  pub fn new() -> Self {
    TwoSum {
      solution: Solution {
        name: String::from("Two Sum"),
        description: String::from(r#"
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.
      "#) },
    }
  }

  fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut one = 0;
    let mut two = 0;
    let mut numbers = HashMap::<i32, i32>::with_capacity(nums.capacity());
    for (i, val) in nums.into_iter().enumerate() {
      if numbers.contains_key(&val) {
        one = *numbers.get(&val).unwrap();
        two = i32::try_from(i).unwrap();
        break;
      } else {
        let index = i32::try_from(i).unwrap();
        numbers.insert(target - val, index);
      }
    }
    vec![one, two]
  }
}

impl Runner for TwoSum {
  // input_format: "nums = [2,7,11,15], target = 9"
  fn run(&self, input: String) {
    println!("Solving {}:", self.solution.name);
    println!("{}", self.solution.description);
    println!();
    let (nums, target) = parse_string(&input);
    println!("nums = {:?}, target = {}", nums, target);
    let result = Self::two_sum(nums, target);
    println!("result = {:?}", result);
  }
}

// A function that takes a string as input and returns a vector of i32 and a i32
fn parse_string(input: &str) -> (Vec<i32>, i32) {
  // Split the input by the comma separator
  let parts: Vec<&str> = input.split(", target = ").collect();

  // Extract the nums part and trim the whitespace and brackets
  let nums_str = parts[0].trim().trim_start_matches("nums = [").trim_end_matches(']');

  // Split the nums part by the space separator and parse each element as i32
  let nums: Vec<i32> = nums_str
      .split(',')
      .map(|s| s.parse::<i32>().unwrap())
      .collect();

  // Extract the target part and trim the whitespace and the target prefix
  let target_str = parts[1].trim();

  // Parse the target part as i32
  let target: i32 = target_str.parse::<i32>().unwrap();

  // Return the nums vector and the target i32
  (nums, target)
}
