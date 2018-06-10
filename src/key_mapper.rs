use std::collections::HashMap;

#[derive(Debug)]
pub struct KeyMapper<'a> {
  mapping: HashMap<&'a str, &'a str>
}

impl<'a> KeyMapper<'a> {
  pub fn new() -> KeyMapper<'a> {
    let mut mapping = HashMap::new();
    mapping.insert("11100001", "1");
    mapping.insert("11010010", "2");
    mapping.insert("11000011", "3");
    mapping.insert("10110100", "4");
    mapping.insert("10100101", "5");
    mapping.insert("10010110", "6");
    mapping.insert("10000111", "7");
    mapping.insert("01111000", "8");
    mapping.insert("01101001", "9");
    mapping.insert("11110000", "0");
    mapping.insert("01011010", "*");
    mapping.insert("01001011", "#");
    KeyMapper { mapping: mapping }
  }

  pub fn key(&self, key: &str) -> Option<&'a str> {
    match self.mapping.get(key) {
      Some(value) => Some(value),
      None => None
    }
  }
}
