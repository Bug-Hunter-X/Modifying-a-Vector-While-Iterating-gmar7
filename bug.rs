fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let mut iter = vec.iter();
    println!("First element: {:?}", iter.next());
    vec.push(3); // Modify the vector while iterating
    println!("Second element: {:?}", iter.next());
}