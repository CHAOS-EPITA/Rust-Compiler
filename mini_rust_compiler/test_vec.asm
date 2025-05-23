section .data
    format_integer db "%d", 0
    format_string db "%s", 0
    newline db 10, 0
    fmt_main_0 db "Test des vecteurs", 10, 0
    fmt_main_2 db "Premier élément: %d", 10, 0
    fmt_main_3 db "Deuxième élément: %d", 10, 0
    fmt_main_4 db "Troisième élément: %d", 10, 0
    fmt_main_5 db "Quatrième élément: %d", 10, 0
    fmt_main_6 db "Cinquième élément: %d", 10, 0
    fmt_main_701 db "values[%d] = %d", 10, 0
    fmt_main_901 db "numbers[%d] = %d", 10, 0

section .text
    extern printf
    extern exit
    global main

main:
    push rbp
    mov rbp, rsp
    sub rsp, 176

    ; println!("Test des vecteurs", ...)

    ; Configuration des registres pour printf
    lea rdi, [rel fmt_main_0]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; Variable declaration: values
    ; Vec literal - creating static array
    mov DWORD [rbp-60], 5  ; Store vec length
    mov rax, 1
    mov DWORD [rbp-64], eax  ; Store vec element 0 at rbp-64
    mov rax, 2
    mov DWORD [rbp-68], eax  ; Store vec element 1 at rbp-68
    mov rax, 3
    mov DWORD [rbp-72], eax  ; Store vec element 2 at rbp-72
    mov rax, 4
    mov DWORD [rbp-76], eax  ; Store vec element 3 at rbp-76
    mov rax, 5
    mov DWORD [rbp-80], eax  ; Store vec element 4 at rbp-80
    lea rax, [rbp-60]  ; Return vec base address (points to length)
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

    ; println!("Quatrième élément: {}", ...)

    ; Évaluation d'un argument
    ; Vector indexing
    mov rax, QWORD [rbp-8]  ; Load vec address values
    push rax  ; Save vec base address
    mov rax, 3
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
    lea rdi, [rel fmt_main_5]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("Cinquième élément: {}", ...)

    ; Évaluation d'un argument
    ; Vector indexing
    mov rax, QWORD [rbp-8]  ; Load vec address values
    push rax  ; Save vec base address
    mov rax, 4
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
    lea rdi, [rel fmt_main_6]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; For loop
    mov rax, 0
    mov DWORD [rbp-16], eax  ; Initialize loop variable
L_for_cond_0:
    mov eax, DWORD [rbp-16]  ; Load counter
    push rax
    mov rax, 5
    mov ecx, eax  ; Move end value to ecx
    pop rax
    cmp eax, ecx  ; Compare counter with end
    jge L_for_end_0  ; Exit if counter >= end

    ; println!("values[{}] = {}", ...)

    ; Évaluation d'un argument
    ; Vector indexing
    mov rax, QWORD [rbp-8]  ; Load vec address values
    push rax  ; Save vec base address
    mov eax, DWORD [rbp-16]  ; Load variable i
    movsx rax, eax  ; Sign extend to 64-bit
    mov rcx, rax  ; Move index to rcx
    pop rax  ; Restore vec base address
    mov rdx, rax         ; Copy vec base address
    add rdx, 4           ; Skip length field
    imul rcx, 4          ; Multiply index by 4 (element size)
    add rdx, rcx         ; Add index offset to get element address
    mov eax, DWORD [rdx] ; Load element value
    movsx rax, eax       ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Évaluation d'un argument
    mov eax, DWORD [rbp-16]  ; Load variable i
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    pop rdx  ; Argument 2
    lea rdi, [rel fmt_main_701]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf
    mov eax, DWORD [rbp-16]  ; Load counter for increment
    add eax, 1  ; Increment
    mov DWORD [rbp-16], eax  ; Store incremented value
    jmp L_for_cond_0  ; Jump back to condition
L_for_end_0:

    ; Variable declaration: numbers
    ; Vec literal - creating static array
    mov DWORD [rbp-60], 3  ; Store vec length
    mov rax, 10
    mov DWORD [rbp-64], eax  ; Store vec element 0 at rbp-64
    mov rax, 20
    mov DWORD [rbp-68], eax  ; Store vec element 1 at rbp-68
    mov rax, 30
    mov DWORD [rbp-72], eax  ; Store vec element 2 at rbp-72
    lea rax, [rbp-60]  ; Return vec base address (points to length)
    mov QWORD [rbp-24], rax

    ; For loop
    mov rax, 0
    mov DWORD [rbp-32], eax  ; Initialize loop variable
L_for_cond_1:
    mov eax, DWORD [rbp-32]  ; Load counter
    push rax
    mov rax, 3
    mov ecx, eax  ; Move end value to ecx
    pop rax
    cmp eax, ecx  ; Compare counter with end
    jge L_for_end_1  ; Exit if counter >= end

    ; println!("numbers[{}] = {}", ...)

    ; Évaluation d'un argument
    ; Vector indexing
    mov rax, QWORD [rbp-24]  ; Load vec address numbers
    push rax  ; Save vec base address
    mov eax, DWORD [rbp-32]  ; Load variable j
    movsx rax, eax  ; Sign extend to 64-bit
    mov rcx, rax  ; Move index to rcx
    pop rax  ; Restore vec base address
    mov rdx, rax         ; Copy vec base address
    add rdx, 4           ; Skip length field
    imul rcx, 4          ; Multiply index by 4 (element size)
    add rdx, rcx         ; Add index offset to get element address
    mov eax, DWORD [rdx] ; Load element value
    movsx rax, eax       ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Évaluation d'un argument
    mov eax, DWORD [rbp-32]  ; Load variable j
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    pop rdx  ; Argument 2
    lea rdi, [rel fmt_main_901]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf
    mov eax, DWORD [rbp-32]  ; Load counter for increment
    add eax, 1  ; Increment
    mov DWORD [rbp-32], eax  ; Store incremented value
    jmp L_for_cond_1  ; Jump back to condition
L_for_end_1:

    ; Épilogue de main avec valeur de retour 0
    mov eax, 0
    mov rsp, rbp
    pop rbp
    ret

