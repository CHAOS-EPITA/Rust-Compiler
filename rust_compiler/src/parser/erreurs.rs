fn erreur_type(elements: Vec<Token>) -> bool
{
    let r = true;
    let mut noms: Vec<String>;
    let mut typ: Vec<String>;
    let mut mutable: Vec<bool>;
    

    for i in elements.len()
    {
        if elements[i] = Keyword("let") //si definition de variable
        {
            if Token::Ident("mut") == elements[i+1] //si mutable
            {
                if Token::Ident(_) == elements[i+2] //si on a le nom de variable   !! ajouter plus tard la verification si le nom n'est pas un mot interdit ou si déjà utiliser
                {
                    if Token::Ident("i32") == elements[i+3] //si on declare un i32
                    {
                        if elements[i+5] == Token::Number(_) //si on a bien un chiffre comme attendu
                        {
                            noms.push(elements[i+2]);
                            typ.push("i32");
                            mutable.push(true);
                        }
                        else // si on a pas i32 alors que declaré comme tel
                        {
                            println!("erreur : vous avez declarer une variable en i32 mais vous avez pas assignez un i32");
                            r = false;
                        }
                    }
                    else if Token::Ident("String") == elements[i+3] //si on déclare un String
                    {
                        if elements[i+5] == Token::StringLit(_) // si on a bien un String comme attendu
                        {
                            noms.push(elements[i+2]);                                                                                                                                                                   typ.push("String");
                            mutable.push(true);
                        }
                        else // si on a pas de string alors que déclaré comme tel
                        {
                            println!("erreur : vous avez declarer une variable en String mais vous avez pas assignez un String");
                        }
                    }
                    //ajouter ici si implementation pour d'autres types
                }

                else //si le nom de variable a un problème
                {
                    println!("vous ne pouvez pas utiliser ce nom de variable");
                }
            }
            else //si non mutable
            {
                if Token::Ident(_) == elements[i+2] //si on a le nom de variable   !! ajouter plus tard la verification si le nom n'est pas un mot interdit ou si déjà utiliser
                {
                    if Token::Ident("i32") == elements[i+3] //si on declare un i32
                    {
                        if elements[i+5] == Token::Number(_) //si on a bien un chiffre comme attendu
                        {
                            noms.push(elements[i+2]);
                            typ.push("i32");
                            mutable.push(false);
                        }
                        else // si on a pas i32 alors que declaré comme tel
                        {
                            println!("erreur : vous avez declarer une variable en i32 mais vous avez pas assignez un i32");
                            r = false;
                        }
                    }
                    else if Token::Ident("String") == elements[i+3] //si on déclare un String
                    {
                        if elements[i+5] == Token::StringLit(_) // si on a bien un String comme attendu
                        {
                            noms.push(elements[i+2]);                                                                                                                                                                   typ.push("String");
                            mutable.push(false);
                        }
                        else // si on a pas de string alors que déclaré comme tel
                        {
                            println!("erreur : vous avez declarer une variable en String mais vous avez pas assignez un String");
                        }
                    }
                    //ajouter ici si implementation pour d'autres types
                }

                else //si le nom de variable a un problème
                {
                    println!("vous ne pouvez pas utiliser ce nom de variable");
                }
            }
        }

        else if elements[i] == Token::Ident(_) && Token::keyword("let") != elements[i-1] // si modification de variable
        {
            for j in noms
            {





