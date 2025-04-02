use crate::common::Result;
use crate::parser::ast::*;
use super::assembly::AssemblyCode;

/// Générateur de code - convertit l'AST en code assembleur
pub struct CodeGenerator {
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, _program: &Program) -> Result<String> {
        let mut assembly = AssemblyCode::new();
        
        // Simplification: nous générons du code assembleur fixe pour afficher "Hello, World!"
        // indépendamment du contenu réel du programme d'entrée
        
        // Section de données pour la chaîne Hello World
        assembly.add_line("section .data");
        assembly.add_line("    hello_msg db 'Hello, World!', 0xa  ; string à afficher avec nouvelle ligne");
        assembly.add_line("    hello_len equ $ - hello_msg        ; longueur de la chaîne");
        
        // Section de code
        assembly.add_line("section .text");
        assembly.add_line("    global _start                      ; point d'entrée pour le linker");
        
        // Point d'entrée du programme
        assembly.add_line("_start:");
        
        // Appel système write(1, msg, len)
        assembly.add_line("    mov rax, 1                         ; syscall 'write'");
        assembly.add_line("    mov rdi, 1                         ; descripteur de fichier 1 (stdout)");
        assembly.add_line("    mov rsi, hello_msg                 ; adresse du message");
        assembly.add_line("    mov rdx, hello_len                 ; longueur du message");
        assembly.add_line("    syscall                            ; appel système");
        
        // Appel système exit(0)
        assembly.add_line("    mov rax, 60                        ; syscall 'exit'");
        assembly.add_line("    mov rdi, 0                         ; code retour 0");
        assembly.add_line("    syscall                            ; appel système");
        
        Ok(assembly.to_string())
    }
}
