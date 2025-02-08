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
    Eq, Neq, Le, Ge, Lt, Gt, // Comparaisons ==, !=, <=, >=, <, >
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


    fn next_char(&mut self) -> Option<char>{}
    fn peek_char(&self) -> Option<char>{}
    fn next_token(&mut self) -> Token{}
    fn consume_comment(&mut self) -> Token{}
    fn consume_iden(&mut self, first: char) -> Token{}
    fn main(){}

}




