    extern crate crossbeam; 

    fn main() { 
        let mut xs = [0, 0, 0, 0]; 

    //crossbeam guarantees that threads will be closed after 
    crossbeam::scope(|scope| { 
    for i in &mut xs { 
        //we increase variable who lives on the stack of another thread
        scope.spawn(move || { 
       *i += 1; // Stack of another thread! 
      }); 
 } 
  }); 
  
    println!("{:?}", xs); 
   }