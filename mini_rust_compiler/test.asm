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
    fmt_main_10 db "%d + %d = %d", 10, 0
    fmt_main_1101 db "i = %d", 10, 0
    fmt_main_12 db "Boucle terminé", 10, 0
    fmt_main_1301 db "j = %d", 10, 0
    fmt_main_14 db "Testing Vec::new()", 10, 0
    fmt_main_16 db "j = %d", 10, 0
    fmt_main_20 db "taille = %d", 10, 0
    fmt_main_2201 db "num[i] = %d", 10, 0

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
    mov eax, DWORD [rbp-8]  ; Load variable x
    movsx rax, eax  ; Sign extend to 64-bit
    push rax
    mov eax, DWORD [rbp-16]  ; Load variable y
    movsx rax, eax  ; Sign extend to 64-bit
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
    mov eax, DWORD [rbp-8]  ; Load variable x
    movsx rax, eax  ; Sign extend to 64-bit
    push rax
    mov eax, DWORD [rbp-16]  ; Load variable y
    movsx rax, eax  ; Sign extend to 64-bit
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
    mov eax, DWORD [rbp-8]  ; Load variable x
    movsx rax, eax  ; Sign extend to 64-bit
    push rax
    mov eax, DWORD [rbp-16]  ; Load variable y
    movsx rax, eax  ; Sign extend to 64-bit
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
    mov eax, DWORD [rbp-8]  ; Load variable x
    movsx rax, eax  ; Sign extend to 64-bit
    push rax
    mov eax, DWORD [rbp-16]  ; Load variable y
    movsx rax, eax  ; Sign extend to 64-bit
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
    sub rsp, 224

    ; Variable declaration: x
    mov rax, 10
    mov QWORD [rbp-8], rax

    ; Variable declaration: y
    mov rax, 20
    mov QWORD [rbp-16], rax

    ; println!("x = {}, y = {}", ...)

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
    lea rdi, [rel fmt_main_2]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("Les opérateurs de base", ...)

    ; Configuration des registres pour printf
    lea rdi, [rel fmt_main_3]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("{} + {} = {}", ...)

    ; Évaluation d'un argument

    ; Appel de fonction: sum()
    ; Sauvegarde des registres volatiles
    push rcx
    push rdx
    push rsi
    push rdi
    mov eax, DWORD [rbp-8]  ; Load variable x
    movsx rax, eax  ; Sign extend to 64-bit
    mov rdi, rax
    mov eax, DWORD [rbp-16]  ; Load variable y
    movsx rax, eax  ; Sign extend to 64-bit
    mov rsi, rax
    call sum
    ; Restauration des registres volatiles
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    push rax  ; Sauvegarde de l'argument sur la pile

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
    pop rcx  ; Argument 3
    lea rdi, [rel fmt_main_4]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("{} - {} = {}", ...)

    ; Évaluation d'un argument

    ; Appel de fonction: diff()
    ; Sauvegarde des registres volatiles
    push rcx
    push rdx
    push rsi
    push rdi
    mov eax, DWORD [rbp-8]  ; Load variable x
    movsx rax, eax  ; Sign extend to 64-bit
    mov rdi, rax
    mov eax, DWORD [rbp-16]  ; Load variable y
    movsx rax, eax  ; Sign extend to 64-bit
    mov rsi, rax
    call diff
    ; Restauration des registres volatiles
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    push rax  ; Sauvegarde de l'argument sur la pile

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
    pop rcx  ; Argument 3
    lea rdi, [rel fmt_main_5]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("{} / {} = {} (division entière)", ...)

    ; Évaluation d'un argument

    ; Appel de fonction: divide()
    ; Sauvegarde des registres volatiles
    push rcx
    push rdx
    push rsi
    push rdi
    mov eax, DWORD [rbp-8]  ; Load variable x
    movsx rax, eax  ; Sign extend to 64-bit
    mov rdi, rax
    mov eax, DWORD [rbp-16]  ; Load variable y
    movsx rax, eax  ; Sign extend to 64-bit
    mov rsi, rax
    call divide
    ; Restauration des registres volatiles
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    push rax  ; Sauvegarde de l'argument sur la pile

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
    pop rcx  ; Argument 3
    lea rdi, [rel fmt_main_6]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; println!("{} * {} = {}", ...)

    ; Évaluation d'un argument

    ; Appel de fonction: mult()
    ; Sauvegarde des registres volatiles
    push rcx
    push rdx
    push rsi
    push rdi
    mov eax, DWORD [rbp-8]  ; Load variable x
    movsx rax, eax  ; Sign extend to 64-bit
    mov rdi, rax
    mov eax, DWORD [rbp-16]  ; Load variable y
    movsx rax, eax  ; Sign extend to 64-bit
    mov rsi, rax
    call mult
    ; Restauration des registres volatiles
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    push rax  ; Sauvegarde de l'argument sur la pile

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
    pop rcx  ; Argument 3
    lea rdi, [rel fmt_main_7]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; Variable declaration: a
    mov rax, 50
    mov QWORD [rbp-24], rax

    ; Variable declaration: b
    mov rax, 20
    mov QWORD [rbp-32], rax

    ; println!("{} + {} = {}", ...)

    ; Évaluation d'un argument

    ; Appel de fonction: sum()
    ; Sauvegarde des registres volatiles
    push rcx
    push rdx
    push rsi
    push rdi
    mov eax, DWORD [rbp-24]  ; Load variable a
    movsx rax, eax  ; Sign extend to 64-bit
    mov rdi, rax
    mov eax, DWORD [rbp-32]  ; Load variable b
    movsx rax, eax  ; Sign extend to 64-bit
    mov rsi, rax
    call sum
    ; Restauration des registres volatiles
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Évaluation d'un argument
    mov eax, DWORD [rbp-32]  ; Load variable b
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Évaluation d'un argument
    mov eax, DWORD [rbp-24]  ; Load variable a
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    pop rdx  ; Argument 2
    pop rcx  ; Argument 3
    lea rdi, [rel fmt_main_10]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; For loop
    mov rax, 0
    mov DWORD [rbp-40], eax  ; Initialize loop variable
L_for_cond_0:
    mov eax, DWORD [rbp-40]  ; Load counter
    push rax
    mov rax, 10
    mov ecx, eax  ; Move end value to ecx
    pop rax
    cmp eax, ecx  ; Compare counter with end
    jge L_for_end_0  ; Exit if counter >= end

    ; println!("i = {}", ...)

    ; Évaluation d'un argument
    mov eax, DWORD [rbp-40]  ; Load variable i
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    lea rdi, [rel fmt_main_1101]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf
    mov eax, DWORD [rbp-40]  ; Load counter for increment
    add eax, 1  ; Increment
    mov DWORD [rbp-40], eax  ; Store incremented value
    jmp L_for_cond_0  ; Jump back to condition
L_for_end_0:

    ; println!("Boucle terminé", ...)

    ; Configuration des registres pour printf
    lea rdi, [rel fmt_main_12]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; For loop
    mov rax, 10
    mov DWORD [rbp-48], eax  ; Initialize loop variable
L_for_cond_1:
    mov eax, DWORD [rbp-48]  ; Load counter
    push rax
    mov rax, 20
    mov ecx, eax  ; Move end value to ecx
    pop rax
    cmp eax, ecx  ; Compare counter with end
    jge L_for_end_1  ; Exit if counter >= end

    ; println!("j = {}", ...)

    ; Évaluation d'un argument
    mov eax, DWORD [rbp-48]  ; Load variable j
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    lea rdi, [rel fmt_main_1301]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf
    mov eax, DWORD [rbp-48]  ; Load counter for increment
    add eax, 1  ; Increment
    mov DWORD [rbp-48], eax  ; Store incremented value
    jmp L_for_cond_1  ; Jump back to condition
L_for_end_1:

    ; println!("Testing Vec::new()", ...)

    ; Configuration des registres pour printf
    lea rdi, [rel fmt_main_14]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; Variable declaration: numbers
    ; Vec::new() - creating empty vector
    mov DWORD [rbp-124], 0  ; Initialize empty vec length
    lea rax, [rbp-124]  ; Return empty vec address
    mov QWORD [rbp-56], rax

    ; println!("j = {}", ...)

    ; Évaluation d'un argument
    ; Vector len method
    mov rax, QWORD [rbp-56]  ; Load vec address numbers
    mov eax, DWORD [rax]  ; Load vector length
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    lea rdi, [rel fmt_main_16]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; Expression statement
    ; Vector push method
    mov rax, QWORD [rbp-56]  ; Load vec address numbers
    push rax  ; Save vec address
    mov rax, 4920
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
    mov rax, QWORD [rbp-56]  ; Load vec address numbers
    push rax  ; Save vec address
    mov rax, 42
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
    mov rax, QWORD [rbp-56]  ; Load vec address numbers
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

    ; println!("taille = {}", ...)

    ; Évaluation d'un argument
    ; Vector len method
    mov rax, QWORD [rbp-56]  ; Load vec address numbers
    mov eax, DWORD [rax]  ; Load vector length
    movsx rax, eax  ; Sign extend to 64-bit
    push rax  ; Sauvegarde de l'argument sur la pile

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    lea rdi, [rel fmt_main_20]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf

    ; Variable declaration: n
    ; Vector len method
    mov rax, QWORD [rbp-56]  ; Load vec address numbers
    mov eax, DWORD [rax]  ; Load vector length
    movsx rax, eax  ; Sign extend to 64-bit
    mov QWORD [rbp-64], rax

    ; For loop
    mov rax, 0
    mov DWORD [rbp-40], eax  ; Initialize loop variable
L_for_cond_2:
    mov eax, DWORD [rbp-40]  ; Load counter
    push rax
    mov eax, DWORD [rbp-64]  ; Load variable n
    movsx rax, eax  ; Sign extend to 64-bit
    mov ecx, eax  ; Move end value to ecx
    pop rax
    cmp eax, ecx  ; Compare counter with end
    jge L_for_end_2  ; Exit if counter >= end

    ; println!("num[i] = {}", ...)

    ; Évaluation d'un argument
    ; Vector indexing
    mov rax, QWORD [rbp-56]  ; Load vec address numbers
    push rax  ; Save vec base address
    mov eax, DWORD [rbp-40]  ; Load variable i
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

    ; Configuration des registres pour printf
    pop rsi  ; Argument 1
    lea rdi, [rel fmt_main_2201]  ; Format string
    xor eax, eax  ; Pas de flottants
    call printf
    mov eax, DWORD [rbp-40]  ; Load counter for increment
    add eax, 1  ; Increment
    mov DWORD [rbp-40], eax  ; Store incremented value
    jmp L_for_cond_2  ; Jump back to condition
L_for_end_2:

    ; Épilogue de main avec valeur de retour 0
    mov eax, 0
    mov rsp, rbp
    pop rbp
    ret

