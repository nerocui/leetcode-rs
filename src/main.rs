mod solution;
mod problems;
use crate::solution::Runner;

fn main() {
    let twosum = problems::two_sum::TwoSum::new();
    twosum.run(String::from("nums = [2,7,11,15], target = 9"));

    let addtwonumbers = problems::add_two_numbers::AddTwoNumbers::new();
    addtwonumbers.run(String::from("l1 = [2,4,3], l2 = [5,6,4]"));
}
