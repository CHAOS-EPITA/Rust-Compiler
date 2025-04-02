/// Classe pour générer et stocker le code assembleur
pub struct AssemblyCode {
    pub code: String,
}

impl AssemblyCode {
    pub fn new() -> Self {
        Self {
            code: String::new(),
        }
    }

    pub fn add_line(&mut self, line: &str) {
        self.code.push_str(line);
        self.code.push('\n');
    }

    pub fn to_string(&self) -> String {
        self.code.clone()
    }
}
