section .data
    format_integer db "%d", 0
    format_string db "%s", 0
    newline db 10, 0
    fmt_main_2 db "Test des conditions if/else", 10, 0
    fmt_main_301 db "%d est plus petit que %d", 10, 0
    fmt_main_302 db "%d est plus grand ou égal à %d", 10, 0
    fmt_main_401 db "x vaut exactement 10", 10, 0
    fmt_main_501 db "y est supérieur à 25", 10, 0
    fmt_main_502 db "y est inférieur ou égal à 25", 10, 0
    fmt_main_601 db "La somme de x et y vaut 30", 10, 0
    fmt_main_60201 db "Et x est inférieur à 15", 10, 0
    fmt_main_60202 db "Mais x est supérieur ou égal à 15", 10, 0

section .text
    extern printf
    extern exit
    global main

main:
    push rbp
    mov rbp, rsp
    sub rsp, 160

    ; Variable declaration: x
    mov rax, 10
    mov QWORD [rbp-8], rax

    ; Variable declaration: y
    mov rax, 20
    mov QWORD [rbp-16], rax

    ; println!("Test des conditions if/else", ...)

    ; Configuration des registres pour printf
    lea rdi, [rel fmt_main_2]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; If statement
    mov eax, DWORD [rbp-8]  ; Load variable x
    movsx rax, eax  ; Sign extend to 64-bit
    push rax
    mov eax, DWORD [rbp-16]  ; Load variable y
    movsx rax, eax  ; Sign extend to 64-bit
    mov rcx, rax
    pop rax
    cmp rax, rcx
    setl al
    movzx rax, al
    test rax, rax  ; Test if condition is zero
    jz L_if_else_0  ; Jump to else if condition is false

    ; Then branch

    ; println!("{} est plus petit que {}", ...)

    ; Évaluation d'un argument
    mov eax, DWORD [rbp-16]  ; Load variable y
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Évaluation d'un argument
    mov eax, DWORD [rbp-8]  ; Load variable x
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    pop rdx  ; Argument 2
    lea rdi, [rel fmt_main_301]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf
    jmp L_if_end_0  ; Skip else branch
L_if_else_0:
    ; Else branch

    ; println!("{} est plus grand ou égal à {}", ...)

    ; Évaluation d'un argument
    mov eax, DWORD [rbp-16]  ; Load variable y
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Évaluation d'un argument
    mov eax, DWORD [rbp-8]  ; Load variable x
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    pop rdx  ; Argument 2
    lea rdi, [rel fmt_main_302]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf
L_if_end_0:

    ; If statement
    mov eax, DWORD [rbp-8]  ; Load variable x
    movsx rax, eax  ; Sign extend to 64-bit
    push rax
    mov rax, 10
    mov rcx, rax
    pop rax
    cmp rax, rcx
    sete al
    movzx rax, al
    test rax, rax  ; Test if condition is zero
    jz L_if_end_1  ; Jump to else if condition is false

    ; Then branch

    ; println!("x vaut exactement 10", ...)

    ; Configuration des registres pour printf
    lea rdi, [rel fmt_main_401]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf
L_if_end_1:

    ; If statement
    mov eax, DWORD [rbp-16]  ; Load variable y
    movsx rax, eax  ; Sign extend to 64-bit
    push rax
    mov rax, 25
    mov rcx, rax
    pop rax
    cmp rax, rcx
    setg al
    movzx rax, al
    test rax, rax  ; Test if condition is zero
    jz L_if_else_2  ; Jump to else if condition is false

    ; Then branch

    ; println!("y est supérieur à 25", ...)

    ; Configuration des registres pour printf
    lea rdi, [rel fmt_main_501]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf
    jmp L_if_end_2  ; Skip else branch
L_if_else_2:
    ; Else branch

    ; println!("y est inférieur ou égal à 25", ...)

    ; Configuration des registres pour printf
    lea rdi, [rel fmt_main_502]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf
L_if_end_2:

    ; If statement
    mov eax, DWORD [rbp-8]  ; Load variable x
    movsx rax, eax  ; Sign extend to 64-bit
    push rax
    mov eax, DWORD [rbp-16]  ; Load variable y
    movsx rax, eax  ; Sign extend to 64-bit
    mov rcx, rax
    pop rax
    add rax, rcx
    push rax
    mov rax, 30
    mov rcx, rax
    pop rax
    cmp rax, rcx
    sete al
    movzx rax, al
    test rax, rax  ; Test if condition is zero
    jz L_if_end_3  ; Jump to else if condition is false

    ; Then branch

    ; println!("La somme de x et y vaut 30", ...)

    ; Configuration des registres pour printf
    lea rdi, [rel fmt_main_601]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; If statement
    mov eax, DWORD [rbp-8]  ; Load variable x
    movsx rax, eax  ; Sign extend to 64-bit
    push rax
    mov rax, 15
    mov rcx, rax
    pop rax
    cmp rax, rcx
    setl al
    movzx rax, al
    test rax, rax  ; Test if condition is zero
    jz L_if_else_4  ; Jump to else if condition is false

    ; Then branch

    ; println!("Et x est inférieur à 15", ...)

    ; Configuration des registres pour printf
    lea rdi, [rel fmt_main_60201]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf
    jmp L_if_end_4  ; Skip else branch
L_if_else_4:
    ; Else branch

    ; println!("Mais x est supérieur ou égal à 15", ...)

    ; Configuration des registres pour printf
    lea rdi, [rel fmt_main_60202]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf
L_if_end_4:
L_if_end_3:

    ; Épilogue de main avec valeur de retour 0
    mov eax, 0
    mov rsp, rbp
    pop rbp
    ret

