fn main() {
    let mut x = 5;
    { // create a new scope
        let y = &mut x;
        *y = 10;
    }
    let z = &mut x;
    *z = 15;
    println!("x: {}", x);
}
// OR
fn main() {
    let mut x = 5;
    let y = x.clone();
    let z = &mut x;
    *z = 10; 
    println!("x: {}", x);
    let z = &mut y; 
    *z = 15;
    println!("y: {}", y);
}