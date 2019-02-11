fn sum(xs: &Vec<i32>) -> i32 {
	xs.iter().sum()
}


fn main() {
    let xs = vec![1,2,3,4];
    let total = sum(&xs);
    println!("sum = {}", total);
}
