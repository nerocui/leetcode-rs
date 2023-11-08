pub struct Solution {
  pub name: String,
  pub description: String,
}

pub trait Runner {
  fn run(&self, input: String);
}
