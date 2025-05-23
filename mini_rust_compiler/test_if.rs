fn main() {
    let x: i32 = 10;
    let y: i32 = 20;
    
    println!("Test des conditions if/else");
    
    if x < y {
        println!("{} est plus petit que {}", x, y);
    } else {
        println!("{} est plus grand ou égal à {}", x, y);
    }
    
    if x == 10 {
        println!("x vaut exactement 10");
    }
    
    if y > 25 {
        println!("y est supérieur à 25");
    } else {
        println!("y est inférieur ou égal à 25");
    }
    
    // Test avec des conditions plus complexes
    if x + y == 30 {
        println!("La somme de x et y vaut 30");
        if x < 15 {
            println!("Et x est inférieur à 15");
        } else {
            println!("Mais x est supérieur ou égal à 15");
        }
    }
}
