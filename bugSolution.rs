fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using a raw pointer, use the vector's indexing. 
    v[0] = 4;
    println!("The first element is: {}", v[0]);
}
