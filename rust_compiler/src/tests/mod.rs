use crate::parser::ast::ASTNode;
use crate::parser::lexer::tokenize;
use crate::parser::parser::Parser;

  
mod test;

fn parse_expression(input: &str) -> Option<ASTNode> {
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[test]
fn test_single_number() {
    let ast = parse_expression("42");
    assert!(matches!(ast, Some(ASTNode::Number(42))));
}

#[test]
fn test_simple_addition() {
    let ast = parse_expression("2 + 3");
    if let Some(ASTNode::Add(left, right)) = ast {
        assert!(matches!(*left, ASTNode::Number(2)));
        assert!(matches!(*right, ASTNode::Number(3)));
    } else {
        panic!("L'AST ne correspond pas à une addition!");
    }
}

#[test]
fn test_simple_multiplication() {
    let ast = parse_expression("4 * 5");
    if let Some(ASTNode::Mul(left, right)) = ast {
        assert!(matches!(*left, ASTNode::Number(4)));
        assert!(matches!(*right, ASTNode::Number(5)));
    } else {
        panic!("L'AST ne correspond pas à une multiplication!");
    }
}

#[test]
fn test_operator_precedence() {
    let ast = parse_expression("2 + 3 * 4");
    if let Some(ASTNode::Add(left, right)) = ast {
        assert!(matches!(*left, ASTNode::Number(2)));
        assert!(matches!(*right, ASTNode::Mul(..)));
        
        if let ASTNode::Mul(mul_left, mul_right) = *right {
            assert!(matches!(*mul_left, ASTNode::Number(3)));
            assert!(matches!(*mul_right, ASTNode::Number(4)));
        } else {
            panic!("Multiplication attendue dans la partie droite!");
        }
    } else {
        panic!("L'AST ne respecte pas la priorité des opérateurs!");
    }
}

#[test]
fn test_parentheses() {
    let ast = parse_expression("(2 + 3) * 4");
    if let Some(ASTNode::Mul(left, right)) = ast {
        assert!(matches!(*right, ASTNode::Number(4)));
        
        if let ASTNode::Add(add_left, add_right) = *left {
            assert!(matches!(*add_left, ASTNode::Number(2)));
            assert!(matches!(*add_right, ASTNode::Number(3)));
        } else {
            panic!("Addition attendue dans les parenthèses!");
        }
    } else {
        panic!("L'AST ne gère pas correctement les parenthèses!");
    }
}

#[test]
fn test_invalid_expression() {
    let ast = parse_expression("2 + + 3");
    assert!(ast.is_none(), "Le parser devrait rejeter les opérateurs consécutifs");
}

#[test]
fn test_unmatched_parentheses() {
    let ast = parse_expression("(2 + 3");
    assert!(ast.is_none(), "Le parser devrait rejeter les parenthèses non fermées");
}

#[test]
fn test_empty_expression() {
    let ast = parse_expression("");
    assert!(ast.is_none(), "Le parser devrait rejeter une expression vide");
}

#[test]
fn test_complex_expression() {
    let ast = parse_expression("2 * (3 + 4 * 5) + 6");
    assert!(ast.is_some(), "Le parser devrait accepter des expressions complexes");
}

#[test]
fn test_whitespace_handling() {
    let ast1 = parse_expression("2+3");
    let ast2 = parse_expression("2 + 3");
    let ast3 = parse_expression("2    +    3");
    
    assert_eq!(ast1, ast2, "Les espaces ne devraient pas affecter le résultat");
    assert_eq!(ast2, ast3, "Les espaces multiples devraient être gérés correctement");
}