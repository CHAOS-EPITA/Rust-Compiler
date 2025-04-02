use std::collections::HashMap;
use std::fmt;

/// Type représente un type spécifique dans notre langage
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Unit,
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "i32"),
            Type::Float => write!(f, "f64"),
            Type::Bool => write!(f, "bool"),
            Type::String => write!(f, "str"),
            Type::Unit => write!(f, "()"),
            Type::Function { params, return_type } => {
                write!(f, "fn(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", return_type)
            }
        }
    }
}

/// TypeEnvironment gère le contexte des types pour les variables et fonctions
#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    scopes: Vec<HashMap<String, Type>>,
}

impl TypeEnvironment {
    /// Crée un nouvel environnement de types vide
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    /// Entre dans un nouveau scope
    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Quitte le scope courant
    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Définit une variable dans le scope courant
    pub fn define(&mut self, name: String, typ: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, typ);
        }
    }

    /// Cherche le type d'une variable en commençant par le scope le plus interne
    pub fn lookup(&self, name: &str) -> Option<&Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(typ) = scope.get(name) {
                return Some(typ);
            }
        }
        None
    }
}

/// Convertit un ast::Type en typer::Type
pub fn from_ast_type(ast_type: &crate::parser::ast::Type) -> Type {
    match ast_type {
        crate::parser::ast::Type::I32 => Type::Int,
        crate::parser::ast::Type::F64 => Type::Float,
        crate::parser::ast::Type::Bool => Type::Bool,
        crate::parser::ast::Type::Str => Type::String,
    }
}
