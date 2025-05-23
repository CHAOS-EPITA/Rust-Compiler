section .data
    format_integer db "%d", 0
    format_string db "%s", 0
    newline db 10, 0
    fmt_main_0 db "Testing vector elements access", 10, 0
    fmt_main_2 db "Vector contents:", 10, 0
    fmt_main_3 db "values[0] = %d", 10, 0
    fmt_main_4 db "values[1] = %d", 10, 0
    fmt_main_5 db "values[2] = %d", 10, 0
    fmt_main_7 db "Initial size: %d", 10, 0
    fmt_main_11 db "After pushing 3 elements, size: %d", 10, 0
    fmt_main_12 db "numbers[0] = %d", 10, 0
    fmt_main_13 db "numbers[1] = %d", 10, 0
    fmt_main_14 db "numbers[2] = %d", 10, 0

section .text
    extern printf
    extern exit
    global main

main:
    push rbp
    mov rbp, rsp
    sub rsp, 160

    ; println!("Testing vector elements access", ...)

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

    ; println!("Vector contents:", ...)

    ; Configuration des registres pour printf
    lea rdi, [rel fmt_main_2]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("values[0] = {}", ...)

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
    lea rdi, [rel fmt_main_3]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("values[1] = {}", ...)

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
    lea rdi, [rel fmt_main_4]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("values[2] = {}", ...)

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
    lea rdi, [rel fmt_main_5]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; Variable declaration: numbers
    ; Vec::new() - creating empty vector
    mov DWORD [rbp-124], 0  ; Initialize empty vec length
    lea rax, [rbp-124]  ; Return empty vec address
    mov QWORD [rbp-16], rax

    ; println!("Initial size: {}", ...)

    ; Évaluation d'un argument
    ; Vector len method
    mov rax, QWORD [rbp-16]  ; Load vec address numbers
    mov eax, DWORD [rax]  ; Load vector length
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    lea rdi, [rel fmt_main_7]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; Expression statement
    ; Vector push method
    mov rax, QWORD [rbp-16]  ; Load vec address numbers
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

    ; Expression statement
    ; Vector push method
    mov rax, QWORD [rbp-16]  ; Load vec address numbers
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

    ; Expression statement
    ; Vector push method
    mov rax, QWORD [rbp-16]  ; Load vec address numbers
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

    ; println!("After pushing 3 elements, size: {}", ...)

    ; Évaluation d'un argument
    ; Vector len method
    mov rax, QWORD [rbp-16]  ; Load vec address numbers
    mov eax, DWORD [rax]  ; Load vector length
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    lea rdi, [rel fmt_main_11]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("numbers[0] = {}", ...)

    ; Évaluation d'un argument
    ; Vector indexing
    mov rax, QWORD [rbp-16]  ; Load vec address numbers
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
    lea rdi, [rel fmt_main_12]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("numbers[1] = {}", ...)

    ; Évaluation d'un argument
    ; Vector indexing
    mov rax, QWORD [rbp-16]  ; Load vec address numbers
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
    lea rdi, [rel fmt_main_13]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("numbers[2] = {}", ...)

    ; Évaluation d'un argument
    ; Vector indexing
    mov rax, QWORD [rbp-16]  ; Load vec address numbers
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
    lea rdi, [rel fmt_main_14]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; Épilogue de main avec valeur de retour 0
    mov eax, 0
    mov rsp, rbp
    pop rbp
    ret

