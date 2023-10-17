pub struct Pair<T> {
  key: String,
  value: T,
}

pub struct Dictionary<T> {
  items: Vec<Pair<T>>,
}

impl<T> Pair<T> {
  pub fn new(key: String, value: T) -> Pair<T> {
    Pair { key, value }
  }

  pub fn key(&self) -> &String {
    &self.key
  }

  pub fn value(&self) -> &T {
    &self.value
  }
}
