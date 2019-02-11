 use std::sync::mpsc::channel; 
  
  fn main() { 
    // transmission and receiver channels
    let (tx, rx) = channel(); 
     
    std::thread::spawn(move || { 
        let xs = rx.recv().unwrap(); 
        println!("{:?}", xs); 
    }); 
    let xs = vec![1, 2, 3, 4]; 
    tx.send(xs).unwrap(); // No copy here! 
    //unwrap says to handle an error in case of rx is closed
 }