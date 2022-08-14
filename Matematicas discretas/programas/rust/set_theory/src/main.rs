fn main() {
    let a: [i32; 4] = [-2, 0, 1, 4];

    for x in a {
        if x > 0 {
            println!("The element: {}", x)
        }
    }
}
