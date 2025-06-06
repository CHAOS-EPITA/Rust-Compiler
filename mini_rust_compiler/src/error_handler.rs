pub struct ErrorHandler {
    file_name: String,
}

impl ErrorHandler {
    pub fn new(file_name: String) -> Self {
        ErrorHandler { file_name }
    }
    
    pub fn report_error(&self, line: usize, message: &str) {
        if line > 0 {
            eprintln!("{}:{} - Erreur à la ligne {}", self.file_name, message, line);
        } else {
            eprintln!("{}: {}", self.file_name, message);
        }
    }
}