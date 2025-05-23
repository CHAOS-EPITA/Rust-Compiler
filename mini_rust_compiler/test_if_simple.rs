fn main() {
    let x: i32 = 10;
    let y: i32 = 20;
    
    println!("Test des conditions if/else");
    
    if x < y {
        println!("x est plus petit que y");
    }
    
    if x == 10 {
        println!("x vaut 10");
    } else {
        println!("x ne vaut pas 10");
    }
    
    if y > 25 {
        println!("y est grand");
    } else {
        println!("y est petit");
    }
}
