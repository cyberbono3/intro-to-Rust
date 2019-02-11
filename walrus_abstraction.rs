struct Walrus { 
  stomach: Vec<Food> 
  } 

trait Animal {
  fn eat(&mut self, food: Food);
}

// конструктор по умолчанию 
trait Default {
    fn default() -> Self;
}
 
impl Animal for Walrus { 
fn eat(&mut self, food: Food) { 
    self.stomach.push(food) 
    } 
}
//статический метод коструктор не может быть привязан к инстансу
impl Default for Walrus { 
fn default() -> Walrus { 
    Walrus {stomach: Vec::new() }
    } 
}

