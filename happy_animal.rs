
//creates happy animal (generic func) with two restrictions  trait Animal and default constructor
//we come up with instance inside the function
fn happy_animal<A: Animal + Default>() -> A { 
    let mut animal = A::default(); 
    let food = make_food(); 
    animal.eat(food); 
    animal //return animal
} 

fn main() { 
    let walrus = happy_animal::<Walrus>(); 
}