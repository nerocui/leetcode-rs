use crate::solution::{ Solution, Runner };
use regex::Regex;

pub struct AddTwoNumbers {
  solution: Solution,
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }

  fn from(input: Vec<i32>) -> Self {
    if input.is_empty() {
      return Self::new(0);
    }
    let mut head = Self::new(input[0]);
    let mut curr = &mut head;
    for i in 1..input.len() {
      curr.next = Some(Box::new(Self::new(input[i])));
      curr = curr.next.as_mut().unwrap();
      if i == input.len() - 1 {
        break;
      }
    }

    head
  }

  fn print(&self) {
    print!("[{}", self.val);
    let mut curr = self.next.as_ref();
    while curr.is_some() {
      print!(", {}", curr.unwrap().val);
      curr = curr.unwrap().next.as_ref();
    }
    print!("]");
  }
}

impl AddTwoNumbers {
  pub fn new() -> Self {
    AddTwoNumbers {
      solution: Solution {
        name: String::from("Add Two Numbers"),
        description: String::from(r#"
You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.
        "#)
      }
    }
  }

  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut carry = false;
    let mut c1 = l1;
    let mut c2 = l2;
    let mut res = ListNode::new(0);
    let mut cres = &mut res;

    while c1.is_some() || c2.is_some() {
      let val1 = match c1.as_ref() {
        Some(c) => c.val,
        None => 0,
      };

      let val2 = match c2.as_ref() {
        Some(c) => c.val,
        None => 0,
      };

      let mut sum = val1 + val2;
      if carry {
        sum += 1;
      }

      if sum > 9 {
        carry = true;
      } else {
        carry = false;
      }

      sum = sum % 10;

      cres.next = Some(Box::new(ListNode::new(sum)));
      cres = cres.next.as_mut().unwrap();

      c1 = match c1 {
        Some(c) => c.next,
        None => None,
      };

      c2 = match c2 {
        Some(c) => c.next,
        None => None,
      };
    }

    if carry {
      cres.next = Some(Box::new(ListNode::new(1)));
    }

    res.next
  }
}

impl Runner for AddTwoNumbers {
  fn run(&self, input: String) {
    println!("Solving {}:", self.solution.name);
    println!("{}", self.solution.description);
    println!();
    let (l1, l2) = parse_string(&input);
    println!("l1 = {:?}, l2 = {:?}", l1, l2);
    let result = Self::add_two_numbers(
      Some(Box::new(ListNode::from(l1))), 
      Some(Box::new(ListNode::from(l2)))
    );
    print!("result = ");
    if let Some(x) = result {
      x.print();
    }
    println!();
  }
}

// A function that takes a string in the format "l1 = [2,4,3], l2 = [5,6,4]"
// and returns two vectors of i32
fn parse_string(s: &str) -> (Vec<i32>, Vec<i32>) {
  // Use regex to match the pattern "l1 = [a,b,c], l2 = [d,e,f]"
  // where a, b, c, d, e, f are integers
  let re = Regex::new(r"l1 = \[(\d+),(\d+),(\d+)\], l2 = \[(\d+),(\d+),(\d+)\]").unwrap();
  // Extract the capture groups from the string
  let caps = re.captures(s).expect("Invalid format");
  // Convert the capture groups to i32 and push them to vectors
  let mut v1 = Vec::new();
  let mut v2 = Vec::new();
  for i in 1..=3 {
      v1.push(caps[i].parse::<i32>().unwrap());
      v2.push(caps[i+3].parse::<i32>().unwrap());
  }
  // Return the vectors
  (v1, v2)
}
