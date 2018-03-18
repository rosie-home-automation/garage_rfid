use std::collections::HashMap;

#[derive(Debug)]
pub struct KeyMapper<'a> {
  mapping: HashMap<&'a str, &'a str>
}

impl<'a> KeyMapper<'a> {
  pub fn new() -> KeyMapper<'a> {
    let mut mapping = HashMap::new();
    mapping.insert("00011110", "1");
    mapping.insert("00101101", "2");
    mapping.insert("00111100", "3");
    mapping.insert("01001011", "4");
    mapping.insert("01011010", "5");
    mapping.insert("01101001", "6");
    mapping.insert("01111000", "7");
    mapping.insert("10000111", "8");
    mapping.insert("10010110", "9");
    mapping.insert("00001111", "0");
    mapping.insert("10100101", "*");
    mapping.insert("10110100", "#");
    KeyMapper { mapping: mapping }
  }

  pub fn key(&self, key: &str) -> Option<&'a str> {
    match self.mapping.get(key) {
      Some(value) => Some(value),
      None => None
    }
  }
}
