section .text
global sum_to_n

sum_to_n:
    mov eax, 0

add_and_decrement:
    add eax, edi
    sub edi, 1
    jg add_and_decrement
    ret
