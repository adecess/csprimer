section .text
global sum_to_n

sum_to_n:
    lea eax, [edi + 1] ; copy edi + 1 to eax
    imul eax, edi ; multiply eax by edi
    shr eax, 1 ; shift eax right once
    ret
