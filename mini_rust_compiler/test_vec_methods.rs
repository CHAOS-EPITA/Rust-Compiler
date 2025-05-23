fn main() {
    println!("Test des méthodes de vecteurs");
    
    // Test avec Vec::new() et .push()
    let mut numbers: Vec<i32> = Vec::new();
    
    println!("Taille initiale: {}", numbers.len());
    
    numbers.push(10);
    println!("Après push(10), taille: {}", numbers.len());
    println!("Premier élément: {}", numbers[0]);
    
    numbers.push(20);
    println!("Après push(20), taille: {}", numbers.len());
    println!("Deuxième élément: {}", numbers[1]);
    
    numbers.push(30);
    println!("Après push(30), taille: {}", numbers.len());
    
    // Afficher tous les éléments
    for i in 0..numbers.len() {
        println!("numbers[{}] = {}", i, numbers[i]);
    }
    
    // Test avec vecteur littéral
    let values: Vec<i32> = [1, 2, 3];
    println!("Vecteur littéral, taille: {}", values.len());
    
    for j in 0..values.len() {
        println!("values[{}] = {}", j, values[j]);
    }
}
