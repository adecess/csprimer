section .text
global sum_to_n

sum_to_n:
    mov eax, 0
    mov ebx, 0
    jmp compare_increment

compare_increment:
    cmp ebx, edi
    jl add_one
    ret

add_one:
    add ebx, 1

add_increment:
    add eax, ebx
    jmp compare_increment
