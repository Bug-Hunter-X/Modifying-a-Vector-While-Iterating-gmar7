fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let vec_clone = vec.clone(); // Create a copy to iterate over
    for element in vec_clone {
        println!("Element: {:?}", element);
    }
    vec.push(3); // Modify the vector after iteration
    println!("Vector after modification: {:?}", vec);
}
