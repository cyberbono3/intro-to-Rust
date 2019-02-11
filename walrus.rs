struct Walrus { 
  stomach: Vec<Food> 
  } 
 impl Walrus { 
 fn eat(&mut self, food: Food) { 
    self.stomach.push(food) 
  } 
}