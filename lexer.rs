/*

            POUR LE MOMENT TOUT EST DANS LE MEME FICHIER FAUDRA RAJOUTER DES PUB DEVANT TOUT ET SUPPRMIMER LE main(){} POUR UTILISER main.rs
            SANY VA À LA NICHE


*/

#[derive(Debug, PartialEq)]


enum Token {
    Ident(String),  // Identificateurs (noms de variables, fonctions...)
    Number(i64),    // Nombres entiers
    StringLit(String), // Chaînes de caractères
    Keyword(String), // Mots-clés (fn, let, if, else...)
    Plus, Minus, Star, Slash, // Opérateurs mathématiques
    Eq, EqEq, Neq, Not, Le, Ge, Lt, Gt, // Comparaisons ==, !=, <=, >=, <, >
    LParen, RParen, LBrace, RBrace, Comma, Semicolon, Colon, Arrow, // Délimiteurs
    Comment(String), // Commentaires
    Eof, // Fin du fichier
    Error(String),
}


struct Lexer{
	input: Vec<char>,
	pos: usize,
}


impl Lexer{


    fn new(input: &str) -> Self{
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }


    fn next_char(&mut self) -> Option<char>{
        if self.pos < self.input.len(){
            let c = self.input[self.pos];
            self.pos += 1;
            Some(c)
        } else {
            None
        }

    }


    fn peek_char(&self) -> Option<char>{
        self.input.get(self.pos).copied()
    }

    fn next_token(&mut self) -> Token{
        while let Some(c) = self.next_char(){
            return match c{
                ' ' | '\t' | '\n' | '\r' => continue,
                '+' => Token::Plus,
                '-' =>{
                    if self.peek_char() == Some('>'){
                        self.next_char();
                        Token::Arrow
                    }
                    else{
                        Token::Minus
                    }
                }

                '*' => Token::Star,
                '/' => {
                    if self.peek_char() == Some('/'){
                        self.consume_comment()
                    } else{
                        Token::Slash
                    }
                }

                '(' => Token::LParen,
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                ',' => Token::Comma,
                ';' => Token::Semicolon,
                ':' => Token::Colon,
                '=' => {
                    if self.peek_char() == Some('='){
                        self.next_char();
                        Token::EqEq
                    }else {
                        Token::Eq
                    }

                }
                '!' => {
                    if self.peek_char() == Some('='){
                        self.next_char();
                        Token::Neq
                    }else {
                        Token::Not
                    }
                }

                '<' => {
                    if self.peek_char() == Some('='){
                        self.next_char();
                        Token::Le
                    }else {
                        Token::Lt
                    }
                }

                '>' => {
                    if self.peek_char() == Some('='){
                        self.next_char();
                        Token::Ge
                    }else{
                        Token::Gt
                    }
                }
                '"' => self.consume_string(),
                '0'..='9' => self.consume_number(c),
                'a'..='z' |'A'..='Z'| '_' => self.consume_ident(c),
                _ => Token::Error(format!("Caractère inattendu: '{}'", c)),
            };

        }
        Token::Eof
    }

    fn consume_comment(&mut self) -> Token {
        let mut comment = String::from("//");
        while let Some(c) = self.next_char(){
            if c == '\n'{
                break;
            }
            comment.push(c);
        }
        Token::Comment(comment)
    }
    fn consume_string(&mut self) -> Token{
        let mut string_lit = String::new();
        while let Some(c) = self.next_char(){
            if c == '"'{
                return Token::StringLit(string_lit);
            }
            string_lit.push(c);
        }
        Token::StringLit(string_lit)    
    }

    fn consume_number(&mut self, first: char) -> Token{
        let mut num = first.to_string();
        while let Some(c) = self.peek_char(){
            if c.is_ascii_digit(){
                num.push(self.next_char().unwrap());
            }else {
                break;
            }

        }
        Token::Number(num.parse().unwrap())
    }

    fn consume_ident(&mut self, first: char) -> Token {
        let mut ident = first.to_string();
        while let Some(c) = self.peek_char(){
            if c.is_alphanumeric() || c == '_'{
                ident.push(self.next_char().unwrap());
            }else{
                break;
            }
        }    

        let keyword = ["fn", "let", "if", "else","while","match"];
        if keyword.contains(&ident.as_str()) {
            Token::Keyword(ident)
        }else{
            Token::Ident(ident)
        }
    }
}

fn main(){
    let source_code = r"
        fn add(a: i32, b: i32) -> i32{
            let mut result: i32 = a + b;
            return result;
         }

        fn sub(a: i32, b:i32) -> i32{
            let mut result: i32 = a - b;
            return result;
         }
    ";

     let mut lexer = Lexer::new(source_code);
     let mut tokens = Vec::new();

     loop{
        let token = lexer.next_token();
        if token == Token::Eof{
            break;
        }
        tokens.push(token);
     }
     // Affiches les tokens dans le terminal
     //.println!("{:?}",source_code);
     println!("{:?}",tokens);
  }


/*
  [Keyword("fn"), Ident("add"), LParen, Ident("a"), Colon, Ident("i32"), Comma, Ident("b"),
   Colon, Ident("i32"), RParen, Arrow, Ident("i32"), LBrace, Keyword("let"), Ident("mut"),
   Ident("result"), Colon, Ident("i32"), Eq, Ident("a"), Plus, Ident("b"), Semicolon,
   Ident("return"),Ident("result"), Semicolon, RBrace, Keyword("fn"), Ident("sub"),
   LParen, Ident("a"), Colon, Ident("i32"),Comma, Ident("b"), Colon, Ident("i32"),
   RParen, Arrow, Ident("i32"), LBrace, Keyword("let"), Ident("mut"),Ident("result"),
   Colon, Ident("i32"), Eq, Ident("a"), Minus, Ident("b"), Semicolon, Ident("return"),
   Ident("result"), Semicolon, RBrace]



*/
