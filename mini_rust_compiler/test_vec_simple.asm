section .data
    format_integer db "%d", 0
    format_string db "%s", 0
    newline db 10, 0
    fmt_main_0 db "Test des vecteurs - version simple", 10, 0
    fmt_main_2 db "Premier élément: %d", 10, 0
    fmt_main_3 db "Deuxième élément: %d", 10, 0
    fmt_main_4 db "Troisième élément: %d", 10, 0

section .text
    extern printf
    extern exit
    global main

main:
    push rbp
    mov rbp, rsp
    sub rsp, 160

    ; println!("Test des vecteurs - version simple", ...)

    ; Configuration des registres pour printf
    lea rdi, [rel fmt_main_0]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; Variable declaration: values
    ; Vec literal - creating static array
    mov DWORD [rbp-64], 3  ; Store vec length
    mov rax, 1
    mov DWORD [rbp-60], eax  ; Store vec element 0
    mov rax, 2
    mov DWORD [rbp-56], eax  ; Store vec element 1
    mov rax, 3
    mov DWORD [rbp-52], eax  ; Store vec element 2
    lea rax, [rbp-64]  ; Return vec base address (points to length)
    mov QWORD [rbp-8], rax

    ; println!("Premier élément: {}", ...)

    ; Évaluation d'un argument
    ; Vector indexing
    mov rax, QWORD [rbp-8]  ; Load vec address values
    push rax  ; Save vec base address
    mov rax, 0
    mov rcx, rax  ; Move index to rcx
    pop rax  ; Restore vec base address
    mov rdx, rax         ; Copy vec base address
    add rdx, 4           ; Skip length field
    imul rcx, 4          ; Multiply index by 4 (element size)
    add rdx, rcx         ; Add index offset to get element address
    mov eax, DWORD [rdx] ; Load element value
    movsx rax, eax       ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    lea rdi, [rel fmt_main_2]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("Deuxième élément: {}", ...)

    ; Évaluation d'un argument
    ; Vector indexing
    mov rax, QWORD [rbp-8]  ; Load vec address values
    push rax  ; Save vec base address
    mov rax, 1
    mov rcx, rax  ; Move index to rcx
    pop rax  ; Restore vec base address
    mov rdx, rax         ; Copy vec base address
    add rdx, 4           ; Skip length field
    imul rcx, 4          ; Multiply index by 4 (element size)
    add rdx, rcx         ; Add index offset to get element address
    mov eax, DWORD [rdx] ; Load element value
    movsx rax, eax       ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    lea rdi, [rel fmt_main_3]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("Troisième élément: {}", ...)

    ; Évaluation d'un argument
    ; Vector indexing
    mov rax, QWORD [rbp-8]  ; Load vec address values
    push rax  ; Save vec base address
    mov rax, 2
    mov rcx, rax  ; Move index to rcx
    pop rax  ; Restore vec base address
    mov rdx, rax         ; Copy vec base address
    add rdx, 4           ; Skip length field
    imul rcx, 4          ; Multiply index by 4 (element size)
    add rdx, rcx         ; Add index offset to get element address
    mov eax, DWORD [rdx] ; Load element value
    movsx rax, eax       ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    lea rdi, [rel fmt_main_4]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; Épilogue de main avec valeur de retour 0
    mov eax, 0
    mov rsp, rbp
    pop rbp
    ret

