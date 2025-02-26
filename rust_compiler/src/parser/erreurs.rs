#[derive(Debug, PartialEq)]
pub enum Token {

    Ident(String),  // Identificateurs (noms de variables, fonctions...)
    Number(i64),    // Nombres entiers
    StringLit(String), // Chaînes de caractères
    Keyword(String), // Mots-clés (fn, let, if, else...)
    Plus, Minus, Star, Slash, // opérateurs mathématique
    Eq, Neq, Le, Ge, Lt, Gt, // Comparaisons ==, !=, <=, >=, <, >
    LParen, RParen, LBrace, RBrace, Comma, Semicolon, Colon, Arrow, // Délimiteurs
    Comment(String), // Commentaires
    Eof, // Fin du fichier
}

pub fn erreur_type(elements: Vec<Token>) -> bool
{
    let mut r = true;
    let mut noms: Vec<String> = Vec::new();
    let mut typ: Vec<String> = Vec::new();
    let mut mutable: Vec<bool> = Vec::new();
    let mut i = 0;
    while i < elements.len(){
        if let Token::Keyword(ref s) = elements[i] {
            if s == "let" //si definition de variable
            {
                if let Token::Ident(ref s) = elements[i+1] //si mutable (si on a bien un token Ident)
                {
                    if s == "mut" //si mutable
                    {
                        if let Token::Ident(ref nom) = elements[i+2] //si on a le nom de variable   !! ajouter plus tard la verification si le nom n'est pas un mot interdit ou si déjà utiliser
                        {
                            if let Token::Ident(ref s) = elements[i+4] //si on déclare un i32 (on verifie que c'est un ident)
                            { 
                                if s == "i32" //si on declare un i32
                                {
                                    if let Token::Number(_) = elements[i+6] //si on a bien un chiffre comme attendu
                                    {
                                        noms.push(nom.clone());
                                        typ.push(String::from("i32"));
                                        mutable.push(true);
                                        i += 6;
                                    }
                                    else // si on a pas i32 alors que declaré comme tel
                                    {
                                        println!("erreur : vous avez declarer une variable en i32 mais vous avez pas assignez un i32");
                                        r = false;
                                        i += 6;
                                    }
                                }
                            }
                            else if let Token::Ident(ref s) = elements[i+4] //si on déclare un string et qu'on a bien un ident
                            { 
                                if s == "String" //si on déclare un String
                                {
                                    if let Token::StringLit(_)  = elements[i+6]// si on a bien un String comme attendu
                                    {
                                        noms.push(nom.clone());
                                        typ.push(String::from("String"));
                                        mutable.push(true);
                                        i += 6
                                    }
                                    else // si on a pas de string alors que déclaré comme tel
                                    {
                                        println!("erreur : vous avez declarer une variable en String mais vous avez pas assignez un String");
                                        i += 6;
                                    }
                                }
                            }
                        }
                        //ajouter ici si implementation pour d'autres types
                        else //si le nom de variable a un problème
                        {
                            println!("vous ne pouvez pas utiliser ce nom de variable");
                            i += 2;
                        }
                    }
                }
                else //si non mutable
                {
                    if let Token::Ident(ref nom) = elements[i+2] //si on a le nom de variable   !! ajouter plus tard la verification si le nom n'est pas un mot interdit ou si déjà utiliser
                    {
                        if let Token::Ident(ref s) = elements[i+3] //si i32 on regarde le Ident
                        {
                            if s == "i32" //si on declare un i32
                            {
                                if let Token::Number(_) = elements[i+5]//si on a bien un chiffre comme attendu
                                {
                                    noms.push(nom.clone());
                                    typ.push(String::from("i32"));
                                    mutable.push(false);
                                    i += 5;
                                }
                                else // si on a pas i32 alors que declaré comme tel
                                {
                                    println!("erreur : vous avez declarer une variable en i32 mais vous avez pas assignez un i32");
                                    r = false;
                                    i += 5;
                                }
                            }
                        }
                        else if let Token::Ident(ref s) = elements[i+3] //si on déclare un String en regardant le Ident
                        { 
                            if s == "String" //si on déclare un String
                            {
                                if let Token::StringLit(_) = elements[i+5]// si on a bien un String comme attendu
                                {
                                    noms.push(nom.clone());
                                    typ.push(String::from("String"));
                                    mutable.push(false);
                                    i += 5;
                                }
                                else // si on a pas de string alors que déclaré comme tel
                                {
                                    println!("erreur : vous avez declarer une variable en String mais vous avez pas assignez un String");
                                    r = false;
                                    i += 5;
                                }
                            }
                        }
                        //ajouter ici si implementation pour d'autres types
                    }
                    else //ce nom n'est pas possible
                    {
                        println!("erreur : vous ne pouvez pas utiliser ce nom de variable");
                        r = false;
                        i += 2;
                    }
                }
            }
        }
        else if let Token::Ident(ref ident_str) = elements[i] {
            if !matches!(elements[i-1], Token::Keyword(ref k) if k == "let") { // si modification de variable
                let mut trouve = false;
                for j in 0..noms.len() {
                    if &noms[j] == ident_str { // si la variable est déjà enregistrer
                        trouve = true;
                        if let Token::Eq = elements[i+1]//si on a une modification de valeur
                        {
                            if let Token::Ident(ref other_ident) = elements[i+2]//si on met la valeur d'une autre variable
                            {
                                //on va verifier si la variable existe et si elle est du bon type
                                let mut trouve2 = false;
                                for k in 0..noms.len() {
                                    if let Token::Ident(ref candidate) = elements[i+2] { //si c'est la bonne variable
                                        if candidate == &noms[k] {
                                            if typ[k] == typ[j] { //si on a le bon type
                                                trouve2 = true;
                                            }
                                            else //le type de la variable n'est pas le bon
                                            {
                                                println!("le type de la variable {:?} ne correspond pas", elements[2]);
                                                r = false;
                                                trouve2 = true;
                                            }
                                        }
                                    }
                                }
                                if trouve2 == false //si la variable n'existe pas
                                {
                                    println!("la variable {:?} n'est pas definit", elements[2]);
                                    r = false;
                                }
                            }
                            else if !matches!(elements[i+2], Token::Number(_))//si on assigne autre chose qu'un chiffre
                            {
                                println!("le type de {:?} ne correspond pas", elements[i+2]);
                                r = false;
                            }
                            //sinon c'est qu'il n'y a pas de soucis
                        }
                        else //si on a pas de = après la variable (le gars fait un truc bizzard)
                        {
                            println!("tu veux faire quoi avec ta variable là ? Relis ton code tu fais de la merde");
                            r = false;
                        }
                        break; //on a bien trouve notre variable où on veut faire des trucs dessus donc pas besoin de continuer à chercher
                    }
                } //fin du for 
                if trouve == false //la variable a modifier n'existe pas
                {
                    println!("la variable {:?} n'est pas définit", elements[i]);
                }
            }
        }
        else //si on a rien a faire 
        {
            i += 1;
        }

    }
    return r;
}

