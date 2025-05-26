# CHAOS Compiler - Un compilateur Rust minimaliste

Ce projet est un compilateur Rust écrit en Rust, développé par l'équipe CHAOS pour le S4 de l'EPITA. Il permet de compiler un sous-ensemble du langage Rust en code assembleur x86_64, puis en exécutable.

## Auteurs

- [@Marcus RABOURDIN](https://github.com/MarcusRabourdin)
- [@Sany BAKIR](https://github.com/TartareSalade)
- [@Louis-Alexandre Charnay](https://github.com/TODO)
- [@Bastian LEBRUN](https://github.com/TODO)

## Installation

Prérequis :
- Rust et Cargo (installés via [rustup](https://rustup.rs/))
- NASM (assembleur)
- GCC (pour l'édition de liens)

Pour installer le compilateur :

```bash
# Cloner le dépôt
git clone https://github.com/TartareSalade/chaos-compiler.git
cd chaos-compiler

# Compiler le projet
cargo build --release
```

## Utilisation

1. Créez un fichier Rust (par exemple `test.rs`) contenant votre code
2. Exécutez le compilateur :

```bash
./target/release/mini_rust_compiler test.rs
```

Le compilateur générera :
- Un fichier assembleur intermédiaire (`test.asm`)
- Un fichier objet intermédiaire (`test.o`)
- Un exécutable final (`test`)

Vous pouvez ensuite exécuter votre programme compilé :

```bash
./test
```

## Fonctionnalités supportées

Notre compilateur prend en charge un sous-ensemble du langage Rust :

### Types de données
- Entiers (`i32`)
- Chaînes de caractères basiques
- Vecteurs (`Vec<i32>`)

### Structures de contrôle
- Expressions conditionnelles (`if`/`else`)
- Boucles `for` avec plages (ex: `for i in 0..10`)

### Fonctions
- Déclaration et appel de fonctions
- Paramètres et valeurs de retour

### Variables
- Déclaration avec `let`
- Variables mutables avec `let mut`

### Opérations
- Opérations arithmétiques (`+`, `-`, `*`, `/`, `%`)
- Comparaisons (`==`, `!=`, `<`, `<=`, `>`, `>=`)

### Entrées/Sorties
- Macro `println!` pour l'affichage

### Vecteurs
- Création avec `Vec::new()` ou via la syntaxe `vec![1, 2, 3]`
- Méthodes `push()` et `len()`
- Accès aux éléments par index (`vec[i]`)

## Exemple de code

Voici un exemple simple de programme que vous pouvez compiler avec notre compilateur :

```rust
fn main() {
    let mut somme = 0;
    
    for i in 0..10 {
        somme = somme + i;
        println!("La somme des nombres de 0 à {} est {}", i, somme);
    }
    
    if somme > 40 {
        println!("La somme est supérieure à 40");
    } else {
        println!("La somme est inférieure ou égale à 40");
    }
    
    let vec = Vec::new();
    let mut vec = vec;
    
    for i in 0..5 {
        vec.push(i * 2);
    }
    
    println!("Taille du vecteur: {}", vec.len());
    println!("Premier élément: {}", vec[0]);
    println!("Dernier élément: {}", vec[4]);
}
```

## Limitations

Ce compilateur est un projet éducatif et ne prend en charge qu'un sous-ensemble du langage Rust. Il n'est pas destiné à une utilisation en production.

## Licence

[MIT](LICENSE)
