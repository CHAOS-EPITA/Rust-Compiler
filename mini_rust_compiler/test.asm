section .data
    format_integer db "%d", 0
    format_string db "%s", 0
    newline db 10, 0
    fmt_main_2 db "x = %d, y = %d", 10, 0
    fmt_main_3 db "Les opérateurs de base", 10, 0
    fmt_main_4 db "%d + %d = %d", 10, 0
    fmt_main_5 db "%d - %d = %d", 10, 0
    fmt_main_6 db "%d / %d = %d (division entière)", 10, 0
    fmt_main_7 db "%d * %d = %d", 10, 0

section .text
    extern printf
    extern exit
    global main

sum:
    push rbp
    mov rbp, rsp
    sub rsp, 16
    mov QWORD [rbp-8], rdi
    mov QWORD [rbp-16], rsi

    ; Return statement
    mov rax, QWORD [rbp-8]  ; Load variable x
    push rax
    mov rax, QWORD [rbp-16]  ; Load variable y
    mov rcx, rax
    pop rax
    add rax, rcx
    mov rsp, rbp
    pop rbp
    ret

    ; Épilogue de la fonction
    mov rsp, rbp
    pop rbp
    ret

diff:
    push rbp
    mov rbp, rsp
    sub rsp, 16
    mov QWORD [rbp-8], rdi
    mov QWORD [rbp-16], rsi

    ; Return statement
    mov rax, QWORD [rbp-8]  ; Load variable x
    push rax
    mov rax, QWORD [rbp-16]  ; Load variable y
    mov rcx, rax
    pop rax
    sub rax, rcx
    mov rsp, rbp
    pop rbp
    ret

    ; Épilogue de la fonction
    mov rsp, rbp
    pop rbp
    ret

divide:
    push rbp
    mov rbp, rsp
    sub rsp, 16
    mov QWORD [rbp-8], rdi
    mov QWORD [rbp-16], rsi

    ; Return statement
    mov rax, QWORD [rbp-8]  ; Load variable x
    push rax
    mov rax, QWORD [rbp-16]  ; Load variable y
    mov rcx, rax
    pop rax
    xor rdx, rdx
    idiv rcx
    mov rsp, rbp
    pop rbp
    ret

    ; Épilogue de la fonction
    mov rsp, rbp
    pop rbp
    ret

mult:
    push rbp
    mov rbp, rsp
    sub rsp, 16
    mov QWORD [rbp-8], rdi
    mov QWORD [rbp-16], rsi

    ; Return statement
    mov rax, QWORD [rbp-8]  ; Load variable x
    push rax
    mov rax, QWORD [rbp-16]  ; Load variable y
    mov rcx, rax
    pop rax
    imul rax, rcx
    mov rsp, rbp
    pop rbp
    ret

    ; Épilogue de la fonction
    mov rsp, rbp
    pop rbp
    ret

main:
    push rbp
    mov rbp, rsp
    sub rsp, 16

    ; Variable declaration: x
    mov rax, 10
    mov QWORD [rbp-8], rax

    ; Variable declaration: y
    mov rax, 20
    mov QWORD [rbp-16], rax

    ; println!("x = {}, y = {}", ...)

    ; Évaluation de l'argument 1
    mov rax, QWORD [rbp-8]  ; Load variable x
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Évaluation de l'argument 2
    mov rax, QWORD [rbp-16]  ; Load variable y
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop r8  ; Récupération de l'argument 2
    pop r9  ; Récupération de l'argument 1
    lea rdi, [rel fmt_main_2]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("Les opérateurs de base", ...)

    ; Configuration des registres pour printf
    lea rdi, [rel fmt_main_3]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("{} + {} = {}", ...)

    ; Évaluation de l'argument 1
    mov rax, QWORD [rbp-8]  ; Load variable x
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Évaluation de l'argument 2
    mov rax, QWORD [rbp-16]  ; Load variable y
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Évaluation de l'argument 3

    ; Appel de fonction: sum()
    ; Sauvegarde des registres volatiles
    push rcx
    push rdx
    push rsi
    push rdi
    mov rax, QWORD [rbp-8]  ; Load variable x
    mov rdi, rax
    mov rax, QWORD [rbp-16]  ; Load variable y
    mov rsi, rax
    call sum
    ; Restauration des registres volatiles
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rcx  ; Récupération de l'argument 3
    pop r8  ; Récupération de l'argument 2
    pop r9  ; Récupération de l'argument 1
    lea rdi, [rel fmt_main_4]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("{} - {} = {}", ...)

    ; Évaluation de l'argument 1
    mov rax, QWORD [rbp-8]  ; Load variable x
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Évaluation de l'argument 2
    mov rax, QWORD [rbp-16]  ; Load variable y
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Évaluation de l'argument 3

    ; Appel de fonction: diff()
    ; Sauvegarde des registres volatiles
    push rcx
    push rdx
    push rsi
    push rdi
    mov rax, QWORD [rbp-8]  ; Load variable x
    mov rdi, rax
    mov rax, QWORD [rbp-16]  ; Load variable y
    mov rsi, rax
    call diff
    ; Restauration des registres volatiles
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rcx  ; Récupération de l'argument 3
    pop r8  ; Récupération de l'argument 2
    pop r9  ; Récupération de l'argument 1
    lea rdi, [rel fmt_main_5]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("{} / {} = {} (division entière)", ...)

    ; Évaluation de l'argument 1
    mov rax, QWORD [rbp-8]  ; Load variable x
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Évaluation de l'argument 2
    mov rax, QWORD [rbp-16]  ; Load variable y
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Évaluation de l'argument 3

    ; Appel de fonction: divide()
    ; Sauvegarde des registres volatiles
    push rcx
    push rdx
    push rsi
    push rdi
    mov rax, QWORD [rbp-8]  ; Load variable x
    mov rdi, rax
    mov rax, QWORD [rbp-16]  ; Load variable y
    mov rsi, rax
    call divide
    ; Restauration des registres volatiles
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rcx  ; Récupération de l'argument 3
    pop r8  ; Récupération de l'argument 2
    pop r9  ; Récupération de l'argument 1
    lea rdi, [rel fmt_main_6]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("{} * {} = {}", ...)

    ; Évaluation de l'argument 1
    mov rax, QWORD [rbp-8]  ; Load variable x
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Évaluation de l'argument 2
    mov rax, QWORD [rbp-16]  ; Load variable y
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Évaluation de l'argument 3

    ; Appel de fonction: mult()
    ; Sauvegarde des registres volatiles
    push rcx
    push rdx
    push rsi
    push rdi
    mov rax, QWORD [rbp-8]  ; Load variable x
    mov rdi, rax
    mov rax, QWORD [rbp-16]  ; Load variable y
    mov rsi, rax
    call mult
    ; Restauration des registres volatiles
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rcx  ; Récupération de l'argument 3
    pop r8  ; Récupération de l'argument 2
    pop r9  ; Récupération de l'argument 1
    lea rdi, [rel fmt_main_7]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; Épilogue de main avec valeur de retour 0
    mov eax, 0
    mov rsp, rbp
    pop rbp
    ret

