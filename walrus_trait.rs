
//we ask compiler to creates impl for traits Default and Clone

 #[derive(Default, Clone)] 
struct Walrus { 
  stomach: Vec<Food> 
  } 

trait Animal {
  fn eat(&mut self, food: Food);
}

//data and interfaces are separated
impl Animal for Walrus { 
fn eat(&mut self, food: Food) { 
    self.stomach.push(food) 
    } 
}

// конструктор по умолчанию 
trait Default {
    fn default() -> Self;
}

trait Close {
    fn clone(&self) -> Self;
}