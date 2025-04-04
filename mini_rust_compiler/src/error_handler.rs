pub struct ErrorHandler {
    file_name: String,
}

impl ErrorHandler {
    pub fn new(file_name: String) -> Self {
        ErrorHandler { file_name }
    }
    
    pub fn report_error(&self, line: usize, message: &str) {
        eprintln!("{}:{}. Erreur à la ligne {}", self.file_name, message, line);
    }
}