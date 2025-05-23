section .data
    format_integer db "%d", 0
    format_string db "%s", 0
    newline db 10, 0
    fmt_main_0 db "Test des méthodes de vecteurs", 10, 0
    fmt_main_2 db "Taille initiale: %d", 10, 0
    fmt_main_4 db "Après push(10), taille: %d", 10, 0
    fmt_main_5 db "Premier élément: %d", 10, 0
    fmt_main_7 db "Après push(20), taille: %d", 10, 0
    fmt_main_8 db "Deuxième élément: %d", 10, 0
    fmt_main_10 db "Après push(30), taille: %d", 10, 0
    fmt_main_1101 db "numbers[%d] = %d", 10, 0
    fmt_main_13 db "Vecteur littéral, taille: %d", 10, 0
    fmt_main_1401 db "values[%d] = %d", 10, 0

section .text
    extern printf
    extern exit
    global main

main:
    push rbp
    mov rbp, rsp
    sub rsp, 176

    ; println!("Test des méthodes de vecteurs", ...)

    ; Configuration des registres pour printf
    lea rdi, [rel fmt_main_0]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; Variable declaration: numbers
    ; Vec::new() - creating empty vector
    mov DWORD [rbp-124], 0  ; Initialize empty vec length
    lea rax, [rbp-124]  ; Return empty vec address
    mov QWORD [rbp-8], rax

    ; println!("Taille initiale: {}", ...)

    ; Évaluation d'un argument
    ; Vector len method
    mov rax, QWORD [rbp-8]  ; Load vec address numbers
    mov eax, DWORD [rax]  ; Load vector length
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    lea rdi, [rel fmt_main_2]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; Expression statement
    ; Vector push method
    mov rax, QWORD [rbp-8]  ; Load vec address numbers
    push rax  ; Save vec address
    mov rax, 10
    mov rcx, rax  ; Save value to push
    pop rax  ; Restore vec address
    mov edx, DWORD [rax]  ; Load current length
    mov r8d, edx  ; Copy length
    imul r8d, 4  ; length * 4
    lea r9, [rax + 4]  ; vec_base + 4 (skip length field)
    add r9, r8  ; Calculate storage address: base + 4 + (length * 4)
    mov DWORD [r9], ecx  ; Store new value
    inc edx  ; Increment length
    mov DWORD [rax], edx  ; Store new length
    ; push() returns void, rax already contains vec address

    ; println!("Après push(10), taille: {}", ...)

    ; Évaluation d'un argument
    ; Vector len method
    mov rax, QWORD [rbp-8]  ; Load vec address numbers
    mov eax, DWORD [rax]  ; Load vector length
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    lea rdi, [rel fmt_main_4]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("Premier élément: {}", ...)

    ; Évaluation d'un argument
    ; Vector indexing
    mov rax, QWORD [rbp-8]  ; Load vec address numbers
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
    lea rdi, [rel fmt_main_5]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; Expression statement
    ; Vector push method
    mov rax, QWORD [rbp-8]  ; Load vec address numbers
    push rax  ; Save vec address
    mov rax, 20
    mov rcx, rax  ; Save value to push
    pop rax  ; Restore vec address
    mov edx, DWORD [rax]  ; Load current length
    mov r8d, edx  ; Copy length
    imul r8d, 4  ; length * 4
    lea r9, [rax + 4]  ; vec_base + 4 (skip length field)
    add r9, r8  ; Calculate storage address: base + 4 + (length * 4)
    mov DWORD [r9], ecx  ; Store new value
    inc edx  ; Increment length
    mov DWORD [rax], edx  ; Store new length
    ; push() returns void, rax already contains vec address

    ; println!("Après push(20), taille: {}", ...)

    ; Évaluation d'un argument
    ; Vector len method
    mov rax, QWORD [rbp-8]  ; Load vec address numbers
    mov eax, DWORD [rax]  ; Load vector length
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    lea rdi, [rel fmt_main_7]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("Deuxième élément: {}", ...)

    ; Évaluation d'un argument
    ; Vector indexing
    mov rax, QWORD [rbp-8]  ; Load vec address numbers
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
    lea rdi, [rel fmt_main_8]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; Expression statement
    ; Vector push method
    mov rax, QWORD [rbp-8]  ; Load vec address numbers
    push rax  ; Save vec address
    mov rax, 30
    mov rcx, rax  ; Save value to push
    pop rax  ; Restore vec address
    mov edx, DWORD [rax]  ; Load current length
    mov r8d, edx  ; Copy length
    imul r8d, 4  ; length * 4
    lea r9, [rax + 4]  ; vec_base + 4 (skip length field)
    add r9, r8  ; Calculate storage address: base + 4 + (length * 4)
    mov DWORD [r9], ecx  ; Store new value
    inc edx  ; Increment length
    mov DWORD [rax], edx  ; Store new length
    ; push() returns void, rax already contains vec address

    ; println!("Après push(30), taille: {}", ...)

    ; Évaluation d'un argument
    ; Vector len method
    mov rax, QWORD [rbp-8]  ; Load vec address numbers
    mov eax, DWORD [rax]  ; Load vector length
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    lea rdi, [rel fmt_main_10]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; For loop
    mov rax, 0
    mov DWORD [rbp-16], eax  ; Initialize loop variable
L_for_cond_0:
    mov eax, DWORD [rbp-16]  ; Load counter
    push rax
    ; Vector len method
    mov rax, QWORD [rbp-8]  ; Load vec address numbers
    mov eax, DWORD [rax]  ; Load vector length
    movsx rax, eax  ; Sign extend to 64-bit
    mov ecx, eax  ; Move end value to ecx
    pop rax
    cmp eax, ecx  ; Compare counter with end
    jge L_for_end_0  ; Exit if counter >= end

    ; println!("numbers[{}] = {}", ...)

    ; Évaluation d'un argument
    ; Vector indexing
    mov rax, QWORD [rbp-8]  ; Load vec address numbers
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
    lea rdi, [rel fmt_main_1101]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf
    mov eax, DWORD [rbp-16]  ; Load counter for increment
    add eax, 1  ; Increment
    mov DWORD [rbp-16], eax  ; Store incremented value
    jmp L_for_cond_0  ; Jump back to condition
L_for_end_0:

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
    mov QWORD [rbp-24], rax

    ; println!("Vecteur littéral, taille: {}", ...)

    ; Évaluation d'un argument
    ; Vector len method
    mov rax, QWORD [rbp-24]  ; Load vec address values
    mov eax, DWORD [rax]  ; Load vector length
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    lea rdi, [rel fmt_main_13]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; For loop
    mov rax, 0
    mov DWORD [rbp-32], eax  ; Initialize loop variable
L_for_cond_1:
    mov eax, DWORD [rbp-32]  ; Load counter
    push rax
    ; Vector len method
    mov rax, QWORD [rbp-24]  ; Load vec address values
    mov eax, DWORD [rax]  ; Load vector length
    movsx rax, eax  ; Sign extend to 64-bit
    mov ecx, eax  ; Move end value to ecx
    pop rax
    cmp eax, ecx  ; Compare counter with end
    jge L_for_end_1  ; Exit if counter >= end

    ; println!("values[{}] = {}", ...)

    ; Évaluation d'un argument
    ; Vector indexing
    mov rax, QWORD [rbp-24]  ; Load vec address values
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
    lea rdi, [rel fmt_main_1401]  ; Format string
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

