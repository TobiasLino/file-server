fn main() {
    println!("Hello, world!");
    let mut x = String::from("Tobias");
    let mut y = x.clone();
    println!("x: {}, y: {}", x, y);
    y = String::from("ana");
    println!("x: {}, y: {}", x, y);
}
