 //dynamic polymorphism
 fn feed_different_animals() { 
         let mut walrus = happy_animal::<Walrus>(); 
         let mut mouse  = happy_animal::<Mouse>(); 

        // dynamic dispatch  работает по интам
        let mut i = 92;

         //does not compile cause walrus and mouse repsresent different typed
        // let animals: Vec<Animal> = vec![walrus, mouse]; 
         let animals: Vec<&mut Animal> = vec![&mut walrus, &mut mouse, &mut i];
         for animal in animals { 
            animal.eat(make_food()); 
        } 
    }