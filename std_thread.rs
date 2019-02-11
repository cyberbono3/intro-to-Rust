fn main() { 
    let xs = vec![1, 2, 3, 4]; 
    // поток может пережить функцию main
    std::thread::spawn( move || { 
    println!("{:?}", xs); 
    }); 
// no xs here
}