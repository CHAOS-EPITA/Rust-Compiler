// Import Diagnostic from miette and re-export SourceSpan
use miette::Diagnostic;
use thiserror::Error;

// Re-export SourceSpan for use in other modules
pub use miette::SourceSpan;

/// Span représente la position d'un élément dans le code source
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn to_source_span(&self) -> SourceSpan {
        (self.start, self.end - self.start).into()
    }
}

// Add implicit conversion from Span to SourceSpan
impl From<Span> for SourceSpan {
    fn from(span: Span) -> Self {
        span.to_source_span()
    }
}

/// Représente toutes les erreurs possibles du compilateur
#[derive(Error, Debug, Diagnostic)]
pub enum CompilerError {
    #[error("Erreur de lexer: {message}")]
    #[diagnostic(code(compiler::lexer_error))]
    LexerError {
        #[source_code]
        src: String,
        #[label("Cette partie a causé une erreur")]
        span: SourceSpan,
        message: String,
    },

    #[error("Erreur de parser: {message}")]
    #[diagnostic(code(compiler::parser_error))]
    ParserError {
        #[source_code]
        src: String,
        #[label("Cette partie a causé une erreur")]
        span: SourceSpan,
        message: String,
    },

    #[error("Erreur de type: {message}")]
    #[diagnostic(code(compiler::type_error))]
    TypeError {
        #[source_code]
        src: String,
        #[label("Cette partie a causé une erreur")]
        span: SourceSpan,
        message: String,
    },

    #[error("Erreur de génération de code: {message}")]
    #[diagnostic(code(compiler::codegen_error))]
    CodegenError {
        message: String,
    },

    #[error("Erreur I/O: {0}")]
    IoError(#[from] std::io::Error),
}

/// Résultat générique pour les opérations du compilateur
pub type Result<T> = std::result::Result<T, CompilerError>;
