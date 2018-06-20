// A WordPositions is a list of the places in a file that a keyword appears
pub struct WordPositions {
  num : i32,
  positions : Vec<usize>,
}

impl WordPositions {
  pub fn new() -> WordPositions {
    WordPositions {
      num : 0,
      positions : Vec::new(),
    }
  }

  pub fn inc(&mut self) {
    self.num += 1;
  }
  
  pub fn num(&self) -> i32 { self.num }

  pub fn add(&mut self, pos : usize) {
    self.positions.push(pos);
  }
}