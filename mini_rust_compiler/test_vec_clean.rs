fn main() {
    println!("Testing vector elements access");
    
    // Test with vector literal
    let values: Vec<i32> = [1, 2, 3];
    
    println!("Vector contents:");
    println!("values[0] = {}", values[0]);
    println!("values[1] = {}", values[1]);
    println!("values[2] = {}", values[2]);
    
    // Test with Vec::new and push
    let mut numbers: Vec<i32> = Vec::new();
    println!("Initial size: {}", numbers.len());
    
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    
    println!("After pushing 3 elements, size: {}", numbers.len());
    println!("numbers[0] = {}", numbers[0]);
    println!("numbers[1] = {}", numbers[1]); 
    println!("numbers[2] = {}", numbers[2]);
}
