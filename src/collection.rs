
enum dataType {
  String,
  Number,
  Boolean,
  Array,
  Object,
  Null,
}

struct Collection {
  //create a data field with a vector of hashmaps
  data: Vec<HashMap<String, dataType>>,
}




impl<T> Collection<T> {
  fn new() -> Self {
    Collection {
      data: Vec::new()
    }
  }

  fn add(&mut self, value: T) {
    let id = self.data.len() as u32;
    self.data.push(CollectionItem { id, value });
  }

  fn get(&self, id: u32) -> Option<&T> {
    self.data.get(id as usize).map(|item| &item.value)
  }

  fn list(&self) -> Vec<&T> {
    self.data.iter().map(|item| &item.value).collect()
  }

  fn del(&mut self, id: u32) {
    self.data.remove(id as usize);
  }

  fn update(&mut self, id: u32, value: T) {
    self.data[id as usize].value = value;
  }

  fn find(&self, predicate: fn(&T) -> bool) -> Option<&T> {
    self.data.iter().find(|item| predicate(&item.value)).map(|item| &item.value)
  }

  //example of find 
  //collection.find(|item| item.id == 1);
  // example of a dog collection 

}

