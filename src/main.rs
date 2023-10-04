use eoa::*;

fn main() {
    let v1 = vec![1, 1, 0, 0, 1, 1, 0, 0, 1, 1];
    let v2 = perturbe(&v1, 1.0 / (v1.len() as f64));
    
    println!("Original:  {:?}", v1);
    println!("Perturbed: {:?}", v2);
}
