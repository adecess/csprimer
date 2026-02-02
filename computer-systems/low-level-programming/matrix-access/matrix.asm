section .text
global matrix
matrix:
    ; rdi: matrix
	; rsi: rows
	; rdx: cols
	; rcx: rindex
	; r8: cindex
	imul rdx, rcx ; Compute cols * rindex
    lea rax, [rdi + rdx*4]; Compute xMatrix + L * (cols * rindex)
	mov eax, [rax + r8*4] ; Read from memory[xMatrix + L * (cols * rindex) + L * cindex]
	ret
