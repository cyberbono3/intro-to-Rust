  fn main() { 
      //rust restricts to pass the link into thread
  let mut xs = [0, 0, 0, 0]; // Stack allocated array 
    
   
   for i in &mut xs { 
        *i += 1; 
   }




   println!("{:?}", xs); 
}