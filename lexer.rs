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
}


struct Lexer{
	input: Vec<char>,
	pos: usize,
}


impl Lexer{


    fn new(input: &str) -> Self{
        Self {
            input: input.chars().collect();
            pos: 0;
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
                ' ' | '\t' | '\r' => continue,
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
                    self.peek_char() == Some('/'){
                        self.consume_comment()
                    } else{
                        Token::Slash;
                    }
                }

                '(' => Token::Lparen,
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                ',' => Token::Comma,
                ';' => Token::Semicolon,
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
                        Token:Not
                    }
                }

                '<' => {
                    if self.peek_char() == Some('='){
                        self.peek_char();
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
                'a'..='z' |'A'..='Z'| '_' => self.consume_iden(c),
                _ => Token::Eof,
            };

        }
        Token::Eof
    }

    fn consume_comment(&mut self) -> Token{}
    fn consume_iden(&mut self, first: char) -> Token{}
    fn main(){}

}




