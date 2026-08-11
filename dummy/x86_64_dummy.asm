global _start

section .text

_start:
	
   mov rax, 1
   mov rdi, 1
   mov rsi, msg
   mov rdx, msglen
   syscall

   mov rax, 2
   mov rdi, cargo
   mov rsi, 0
   mov rdx, 0
   syscall

 ;   mov rax, 0
;    push rax

  ;  loop:
   ; mov rax, 1
;    mov rdi, 1
 ;   mov rsi, msg
  ;  mov rdx, msglen
;    syscall

 ;   pop rax
  ;  cmp rax, 10
 ;   je exit
;    inc rax
  ;  push rax

   ; jmp loop

    ;exit:
  mov rax, 60
  mov rdi, 0
  syscall


section .rodata
  cargo: db "Cargo.toml", 0
  msg: db "Hello, Dummy!", 10
  msglen: equ $ - msg


