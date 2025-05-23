fn main() {
    println!("Test des vecteurs");
    
    // Test with vector literal (this works)
    let values: Vec<i32> = [1, 2, 3, 4, 5];
    
    println!("Premier élément: {}", values[0]);
    println!("Deuxième élément: {}", values[1]);
    println!("Troisième élément: {}", values[2]);
    println!("Quatrième élément: {}", values[3]);
    println!("Cinquième élément: {}", values[4]);
    
    // Test with for loop
    for i in 0..5 {
        println!("values[{}] = {}", i, values[i]);
    }
    
    // Test with another vector
    let numbers: Vec<i32> = [10, 20, 30];
    
    for j in 0..3 {
        println!("numbers[{}] = {}", j, numbers[j]);
    }
}
