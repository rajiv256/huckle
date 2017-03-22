global long_mode_start

section .text
bits 64
long_mode_start:
    
	mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    extern rust_main     ; new
    call rust_main       ; new

    .os_returned:
    ; rust main returned, print `OS returned!`
    mov rax, 0x2f722f202f532f4f
    mov [0xb8000], rax
    mov rax, 0x2f722f752f742f65
    mov [0xb8008], rax
    mov rax, 0x2f212f642f652f6e
    mov [0xb8010], rax
    hlt