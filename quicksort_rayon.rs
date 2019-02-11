    extern crate rayon; 

    fn quick_sort(xs: &mut[i32]) { 
        if xs.len() <= 1 { return } 
        //acepts mutalbe link to the array 
        let mid = partition(xs); 
        //partions into two subarrays
        let (lo, hi) = xs.split_at_mut(mid); 
        //execute two closures in parallel
        rayon::join(|| quick_sort(lo), || quick_sort(hi)); 
    } 
    fn partition(xs: &mut[i32]) -> usize { /* … */ } 
    fn main() { 
    let mut xs = [1, 3, 0, 6, 2, 4, 92]; 
    quick_sort(&mut xs); 
}