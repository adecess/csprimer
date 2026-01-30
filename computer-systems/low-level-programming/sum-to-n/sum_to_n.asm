section .text
global sum_to_n

sum_to_n:
    mov eax, 0
    mov ebx, 0

compare_increment:
    cmp ebx, edi
    jl add_one_and_increment
    ret

add_one_and_increment:
    add ebx, 1
    add eax, ebx
    jmp compare_increment
