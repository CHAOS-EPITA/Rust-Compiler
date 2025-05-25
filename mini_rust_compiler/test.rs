fn main() {

    let x:i32 = 10;
    let y:i32 = 20;
    println!("x = {}, y = {}",x,y);
    println!("Les opérateurs de base");
    println!("{} + {} = {}",x,y,sum(x,y));
    println!("{} - {} = {}",x,y,diff(x,y));
    println!("{} / {} = {} (division entière)",x,y,divide(x,y));
    println!("{} * {} = {}",x,y,mult(x,y));



    let a:i32 = 50;
    let b:i32 = 20;
    println!("{} + {} = {}",a,b,sum(a,b));

    for i in 0..10 {
        println!("i = {}", i);
    }
    println!("Boucle terminé");

    for j in 10..20 {
        println!("j = {}", j);
    }

    println!("Testing Vec::new()");
    
    let mut numbers: Vec<i32> = Vec::new();
    println!("j = {}", numbers.len()); 
    numbers.push(4920);
    numbers.push(42);
    numbers.push(10);
    println!("taille = {}", numbers.len()); 
    let n = numbers.len();
    for i in 0..n{
        println!("num[i] = {}", numbers[i]);
    }

}


fn sum(x:i32,y:i32) ->i32{ return x+y;}
fn diff(x:i32,y:i32) ->i32{ return x-y;}
fn divide(x:i32,y:i32) ->i32{ return x/y;}
fn mult(x:i32,y:i32) ->i32{ return x*y;}



