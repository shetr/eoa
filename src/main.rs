use eoa::*;

fn main() {
    let v1 = [1, 1, 0, 0, 1, 1, 0, 0, 1, 1];
    let v2 = perturbe(&v1, 1.0 / (v1.len() as f64));
    let bounds = [Bounds { lower: 0.0, upper: 1.0 }, Bounds { lower: 0.0, upper: 31.0 }];
    let v3 = bin_to_real(&v1, &bounds);
    
    println!("Original:  {:?}", v1);
    println!("Perturbed: {:?}", v2);
    println!("Real: {:?}", v3);
}
